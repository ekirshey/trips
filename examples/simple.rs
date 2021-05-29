use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

extern crate trips;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut renderer = trips::Renderer::new(&window);
    
    let mut mesh_obj = renderer.load_mesh("res/Box.gltf");
    let mut monkey = renderer.load_mesh("res/monkey.gltf");

    mesh_obj.material.render_properties.albedo = glam::Vec4::new(1.0,0.0,1.0,1.0);
    monkey.material.render_properties.albedo = glam::Vec4::new(1.0,1.0,1.0,1.0);
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let mut scene = trips::Scene {
                    meshes: Vec::new()
                };
                scene.meshes.push(&mesh_obj);
                scene.meshes.push(&monkey);
                renderer.update();
                match renderer.draw(&scene) {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => renderer.rebuild_swapchain(),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
    
}