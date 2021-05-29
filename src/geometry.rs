use std::{mem::size_of};
use std::marker::PhantomData;

const STARTING_VERTICES: usize = 1 << 16;
const STARTING_INDICES: usize = 1 << 16;

#[derive(Debug, Copy, Clone)]
pub struct BufferRange<T: bytemuck::Pod> {
    pub start: wgpu::BufferAddress,
    pub size: usize,
    pub buffer_item_size: usize, // need a better name or is it even needed?
    phantom: PhantomData<T>
}

pub struct Buffer<T: bytemuck::Pod> {
    wgpu_buffer: wgpu::Buffer,
    buffer_offset: u64,
    idx_offset: u64,
    phantom: PhantomData<T>
}

impl<T: bytemuck::Pod> Buffer<T> {
    pub fn new(buffer: wgpu::Buffer) -> Self {
        Self {
            wgpu_buffer: buffer,
            buffer_offset: 0,
            idx_offset: 0,
            phantom: PhantomData
        }
    }

    pub fn write(&mut self, queue: &wgpu::Queue, data: &Vec<T>) -> BufferRange<T> {
        queue.write_buffer(
            &self.wgpu_buffer,
            self.buffer_offset,
            bytemuck::cast_slice(data),
        );

        let data_size = (data.len() * size_of::<T>()) as wgpu::BufferAddress;
        self.buffer_offset = self.buffer_offset + data_size;
        let start = self.idx_offset;
        self.idx_offset = self.idx_offset + data.len() as u64;

        let buffer_item_size = size_of::<T>();
        BufferRange {
            start,
            size: data.len(),
            buffer_item_size,
            phantom: PhantomData
        }
    }
}

pub struct Geometry {
    pub vertex_positions: Vec<glam::Vec3>,
    pub indices: Vec<u32>
}

#[derive(Debug, Copy, Clone)]
pub struct GeometryHandle {
    pub vertex_position_range: BufferRange<glam::Vec3>,
    pub indices_range: BufferRange<u32>,
    pub geometry_idx: usize
}

pub struct GeometryStore {
    pub vertex_positions: Buffer<glam::Vec3>,
    pub indices: Buffer<u32>,
    pub geometries: Vec<Geometry>
}

impl GeometryStore {
    pub fn new(device: &wgpu::Device) -> Self {
        let vertex_positions = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&format!("Vertex Positions")),
                size: STARTING_VERTICES as u64,
                usage: wgpu::BufferUsage::COPY_DST |wgpu::BufferUsage::VERTEX,
                mapped_at_creation: false // not sure
            }
        );

        let indices = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&format!("Indices")),
                size: STARTING_INDICES as u64,
                usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::INDEX,
                mapped_at_creation: false
            }
        );

        Self {
            vertex_positions: Buffer::new(vertex_positions),
            indices: Buffer::new(indices),
            geometries: Vec::new()
        }
    }

    pub fn load_mesh(&mut self, queue: &wgpu::Queue, geometry: Geometry) -> GeometryHandle {
        let vertex_position_range = self.vertex_positions.write(queue, &geometry.vertex_positions);
        let indices_range = self.indices.write(queue, &geometry.indices);
        self.geometries.push(geometry);
        let geometry_idx = self.geometries.len()-1;

        GeometryHandle {
            vertex_position_range,
            indices_range,
            geometry_idx
        }
    }

    pub fn get_geometry_data<'a>(&'a self, handle: &GeometryHandle) -> &'a Geometry{
        &self.geometries[handle.geometry_idx]
    }

    pub fn set_geometry_buffers<'a, 'b>(&'a self, render_pass: &mut wgpu::RenderPass<'b>) 
        where 'a: 'b
    {
        render_pass.set_vertex_buffer(0, self.vertex_positions.wgpu_buffer.slice(..));
        render_pass.set_index_buffer(self.indices.wgpu_buffer.slice(..), wgpu::IndexFormat::Uint32);
    }
}