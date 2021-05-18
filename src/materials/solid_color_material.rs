use wgpu::util::DeviceExt;
use crate::pipelines::{
    PipelineConfig
};
use crate::shaders::{ShaderType};
use crate::WGPUState;
use crate::materials::{
    RenderProperties,
    MaterialBuffers
};

pub struct SolidColorMaterial {}

impl SolidColorMaterial {
    pub fn get_pipeline_config(renderer_state: &WGPUState) -> PipelineConfig {
        let uniform_bind_group_layout =
            renderer_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        PipelineConfig {
            vert_shader: ShaderType::BasicVert,
            frag_shader: Some(ShaderType::BasicFrag),
            uniform_buffer_layout: Some(uniform_bind_group_layout)
        }
    }

    pub(in crate::materials) fn create_buffers(renderer_state: &WGPUState, render_properties: &RenderProperties) -> MaterialBuffers {
        let uniform_buffer = renderer_state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&render_properties.albedo.to_array()),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let uniform_bind_group_layout = renderer_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });
        
        let uniform_bind_group = renderer_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        MaterialBuffers {
            uniform_buffer,
            uniform_bind_group
        }   
    }

}