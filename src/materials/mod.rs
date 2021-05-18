use crate::WGPUState;
use crate::pipelines::PipelineType;

mod solid_color_material;

pub use solid_color_material::SolidColorMaterial;
pub type MaterialHandle = usize;

#[derive(Debug, Copy, Clone)]
pub struct RenderProperties {
    pub albedo: glam::Vec4
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub(crate) material_handle: MaterialHandle,
    pub material_type: MaterialType,
    pub render_properties: RenderProperties
}

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    SolidColorMaterial
}

pub struct MaterialBuffers {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup
}

impl Material {

    pub(crate) fn create_buffers(renderer_state: &WGPUState, material_type: &MaterialType, render_properties: &RenderProperties) -> MaterialBuffers {
        match material_type {
            MaterialType::SolidColorMaterial => {
                return SolidColorMaterial::create_buffers(renderer_state, &render_properties);
            }
        }
    }

    pub(crate) fn new(material_handle: MaterialHandle,
                      material_type: MaterialType,  
                      render_properties: RenderProperties) -> Self 
    {
        Self {
            material_handle,
            material_type,
            render_properties
        }
    }

    pub(crate) fn get_pipeline_id(&self) -> PipelineType {
        match &self.material_type {
            MaterialType::SolidColorMaterial=> {
                return PipelineType::SolidColorMaterial
            }
        }
    }

}