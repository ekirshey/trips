use crate::mesh;

#[derive(Debug, Copy, Clone)]
pub struct Object {
    pub object_type: ObjectType
}

#[derive(Debug, Copy, Clone)]
pub struct Light {

}

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Mesh(mesh::Mesh),
    Light(Light)
}