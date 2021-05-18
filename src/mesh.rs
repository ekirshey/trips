use crate::geometry::{
    GeometryHandle
};
use crate::materials::{
    Material,
    MaterialType
};

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
}