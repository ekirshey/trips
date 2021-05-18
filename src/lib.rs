
use winit::window::Window;
use futures::executor::block_on;
use wgpu::util::DeviceExt;
use std::iter;

mod wgpu_state;
mod shaders;
mod buffers;
mod pipelines;
mod object;
mod scene;
mod materials;
mod geometry;
mod mesh;

// Exports
pub use scene::Scene;
pub use materials::{
    MaterialType,
    SolidColorMaterial
};
pub use object::{ Object, ObjectType };
use mesh::Mesh;
use geometry::{ Geometry, GeometryBuffers };
use wgpu_state::WGPUState;
use shaders::ShaderStore;
use pipelines::PipelineStore;
use materials::{
    Material,
    MaterialBuffers,
    RenderProperties
};

pub struct Renderer {
    state: WGPUState,
    shader_store: ShaderStore,
    pipeline_store: PipelineStore,
    material_buffers: Vec<MaterialBuffers>,
    geometry_buffers: Vec<GeometryBuffers>,
    geometries: Vec<Geometry>
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let state = block_on(WGPUState::new(window));
        let shader_store = ShaderStore::new(&state);
        let pipeline_store = PipelineStore::new(&state, &shader_store);
        Self {
            state,
            shader_store,
            pipeline_store,
            material_buffers: Vec::new(),
            geometry_buffers: Vec::new(),
            geometries: Vec::new()
        }
    }

    pub fn rebuild_swapchain(&mut self) {
        self.state.resize(self.state.size);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.state.resize(new_size);
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&mut self, scene: &Scene) -> Result<(), wgpu::SwapChainError>
    {
        let frame = self.state.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            
            for obj in &scene.objects {
                match &obj.object_type {
                    object::ObjectType::Mesh(mesh) => {
                        let material_buffers = &self.material_buffers[mesh.material.material_handle];
                        let pipeline = &self.pipeline_store.get(mesh.material.get_pipeline_id());
                        render_pass.set_pipeline(pipeline);
                        render_pass.set_bind_group(0, &material_buffers.uniform_bind_group, &[]);
                        
                        let geometry = &self.geometries[mesh.geometry];
                        let geometry_buffer = &self.geometry_buffers[geometry.buffer_idx];
                        render_pass.set_vertex_buffer(0, geometry_buffer.vertex_positions.slice(..));
                        render_pass.set_index_buffer(geometry_buffer.indices.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..geometry.indices.len() as u32, 0, 0..1);
                    },
                    _ => {}
                }
            }
        }

        self.state.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }

    pub fn create_material(&mut self, material_type: MaterialType, render_properties: RenderProperties) -> Material {
        let material_buffers = Material::create_buffers(&self.state, &material_type, &render_properties);
        self.material_buffers.push(material_buffers);
        return Material::new(
            self.material_buffers.len()-1,
            material_type,
            render_properties
        )
    }

    pub fn load_mesh(&mut self, file: &str) -> Object {
        let (doc, buffers, images) = gltf::import(file).unwrap();
        let mesh_data = doc.meshes().next().expect("no meshes in data.glb");

        let primitive = mesh_data.primitives().next().expect("no primitives in data.glb");
        let reader = primitive.reader(|b| Some(&buffers.get(b.index())?.0[..b.length()]));
    
        let vertex_positions: Vec<_> = reader.read_positions().unwrap().map(glam::Vec3::from).collect();
        let indices = reader.read_indices().unwrap().into_u32().collect::<Vec<_>>();
        let vertex_buffer = self.state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file)),
                contents: bytemuck::cast_slice(&vertex_positions),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );
        
        let index_buffer = self.state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", file)),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        self.geometry_buffers.push(GeometryBuffers{
            vertex_positions: vertex_buffer,
            indices: index_buffer
        });

        // Probably want to move this into some shared space as well
        self.geometries.push(Geometry {
            vertex_positions: vertex_positions,
            indices: indices,
            buffer_idx: self.geometry_buffers.len()-1
        });

        let render_properties = RenderProperties {
            albedo: glam::Vec4::new(1.0,1.0,0.0,1.0)
        };

        Object {
            object_type: ObjectType::Mesh({
                Mesh::new(self.geometries.len()-1, 
                    self.create_material(MaterialType::SolidColorMaterial, render_properties)
                )
            })
        }
    }
}
