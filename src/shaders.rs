use std::collections::HashMap;
use crate::WGPUState;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ShaderType {
    BasicVert,
    BasicFrag
}

pub struct ShaderStore {
    store: HashMap<ShaderType, wgpu::ShaderModule>
}

impl ShaderStore {
    pub fn new(renderer_state: &WGPUState) -> Self {
        let mut store = HashMap::new();
    
        store.insert(ShaderType::BasicVert,
                     renderer_state.device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv")));
        store.insert(ShaderType::BasicFrag,
                     renderer_state.device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv")));
        
        Self {
            store
        }    
    }

    pub fn get(&self, shader_type: ShaderType) -> &wgpu::ShaderModule {
        self.store.get(&shader_type).unwrap()
    }
}

