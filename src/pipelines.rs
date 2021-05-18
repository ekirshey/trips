use std::collections::HashMap;
use wgpu::util::DeviceExt;
use crate::WGPUState;
use crate::shaders::{ShaderStore, ShaderType};
use crate::materials::SolidColorMaterial;

pub struct PipelineConfig {
    pub vert_shader: ShaderType,
    pub frag_shader: Option<ShaderType>,
    pub uniform_buffer_layout: Option<wgpu::BindGroupLayout>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum PipelineType {
    SolidColorMaterial
}

pub struct PipelineStore {
    store: HashMap<PipelineType, wgpu::RenderPipeline>
}

impl PipelineStore {
    pub fn new(renderer_state: &WGPUState, shaders: &ShaderStore) -> Self {
        let mut store = HashMap::new();
    
        let pipeline_config = SolidColorMaterial::get_pipeline_config(renderer_state);        
        store.insert(PipelineType::SolidColorMaterial, 
                     PipelineStore::create_pipeline(renderer_state, shaders, pipeline_config));
        
        Self {
            store
        }
    }

    pub fn get(&self, pipeline_type: PipelineType) -> &wgpu::RenderPipeline{
        self.store.get(&pipeline_type).unwrap()
    }

    fn create_pipeline(renderer_state : &WGPUState, 
                       shader_store: &ShaderStore,
                       pipeline_config: PipelineConfig) -> wgpu::RenderPipeline              
    {

        let mut layouts = Vec::<&wgpu::BindGroupLayout>::new();
        if let Some(ref uniform_buffer_layout) = pipeline_config.uniform_buffer_layout {
            layouts.push(uniform_buffer_layout);
        }

        let render_pipeline_layout =
            renderer_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &layouts,
                push_constant_ranges: &[],
            });
        
        let swapchain_format : [wgpu::ColorTargetState; 1] = [renderer_state.swapchain_format.into()];
        let fragment_shader_module = if let Some(frag_shader) = pipeline_config.frag_shader {
            Some(wgpu::FragmentState {
                module: &shader_store.get(frag_shader),
                entry_point: "main",
                targets: &swapchain_format
            })
        }
        else {
            None
        };

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        };

        let render_pipeline = renderer_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_store.get(pipeline_config.vert_shader),
                    entry_point: "main", 
                    buffers: &[
                        vertex_buffer_layout
                        //InstanceRaw::desc()
                    ], 
                },
                fragment: fragment_shader_module,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, 
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1, 
                    mask: !0, 
                    alpha_to_coverage_enabled: false, 
                },
        });

        render_pipeline
    }
}