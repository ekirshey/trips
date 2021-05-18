pub type GeometryHandle = usize;

pub struct Geometry {
    pub vertex_positions: Vec<glam::Vec3>,
    pub indices: Vec<u32>,
    pub buffer_idx: usize
}

pub struct GeometryBuffers {
    pub vertex_positions: wgpu::Buffer,
    pub indices: wgpu::Buffer
}