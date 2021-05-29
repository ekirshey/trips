use crate::geometry::{
    Geometry,
    GeometryStore,
    GeometryHandle
};
use crate::materials::{
    Material,
    MaterialType
};
use crate::pipelines::PipelineStore;
use std::{mem::size_of_val};

#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub geometry: GeometryHandle,
    pub material: Material
}

impl Mesh {
    pub fn new(geometry: GeometryHandle, material: Material) -> Self {
        Self {
            geometry,
            material
        }
    }

    pub fn render<'a, 'b>(&'a self, 
                          renderpass: &mut wgpu::RenderPass<'b>)
    {
        let start = self.geometry.indices_range.start as u32;
        let end = start + self.geometry.indices_range.size as u32;
        let offset = self.geometry.vertex_position_range.start as i32;
        renderpass.draw_indexed(start..end, offset, 0..1);
    }   
}