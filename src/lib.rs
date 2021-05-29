 
use winit::window::Window;
use futures::executor::block_on;
use wgpu::util::DeviceExt;
use std::iter;

mod wgpu_state;
mod shaders;
mod pipelines;
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
use mesh::Mesh;
use geometry::{ Geometry, GeometryStore };
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
    geometry_store: GeometryStore
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let state = block_on(WGPUState::new(window));
        let shader_store = ShaderStore::new(&state);
        let pipeline_store = PipelineStore::new(&state, &shader_store);
        let geometry_store = GeometryStore::new(&state.device);

        Self {
            state,
            shader_store,
            pipeline_store,
            material_buffers: Vec::new(),
            geometry_store
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

            for mesh in &scene.meshes {
                let material_buffers = &self.material_buffers[mesh.material.material_handle];
                let pipeline = &self.pipeline_store.get(mesh.material.get_pipeline_id()); //1
                render_pass.set_pipeline(pipeline); //1
                render_pass.set_bind_group(0, &material_buffers.uniform_bind_group, &[]); 
                self.geometry_store.set_geometry_buffers(&mut render_pass);

                mesh.render(&mut render_pass);
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

    // I only need 2 buffers. One for material and one for vertex stuff.
    pub fn load_mesh(&mut self, file: &str) -> Mesh {
        let (doc, buffers, images) = gltf::import(file).unwrap();
        let mesh_data = doc.meshes().next().expect("no meshes in data.glb");

        let primitive = mesh_data.primitives().next().expect("no primitives in data.glb");
        let reader = primitive.reader(|b| Some(&buffers.get(b.index())?.0[..b.length()]));
    
        let vertex_positions: Vec<_> = reader.read_positions().unwrap().map(glam::Vec3::from).collect();
        let indices = reader.read_indices().unwrap().into_u32().collect::<Vec<_>>();

        let geometry = Geometry {
            vertex_positions: vertex_positions,
            indices: indices
        };
        
        let geometry_handle = self.geometry_store.load_mesh(&self.state.queue, geometry);

        let render_properties = RenderProperties {
            albedo: glam::Vec4::new(1.0,1.0,0.0,1.0)
        };

        mesh::Mesh::new(
            geometry_handle,
            self.create_material(MaterialType::SolidColorMaterial, render_properties)
        )
    }
}
