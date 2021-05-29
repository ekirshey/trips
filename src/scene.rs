use crate::Mesh;

// Do I just want to make Meshes as a trait and then light as a separate array in the scene?
// It might be much easier
// Mesh trait which exposes geometry, material and children and then we can have InstancedMesh, AnimatedMesh, whatever
pub struct Scene<'a> {
    pub meshes: Vec<&'a Mesh>,
    //pub lights: Vec<&'a dyn Object>
}