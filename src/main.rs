use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use futures::executor::block_on;

async fn run(event_loop: EventLoop<()>, _window: Window, _swapchain_format: wgpu::TextureFormat) {
    println!("Started running.");

    event_loop.run(
        move |event: Event<()>, _, controlflow: &mut ControlFlow| match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => *controlflow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        state: ElementState::Pressed,
                        ..
                    } => println!("Clicked spacebar."),
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
    );
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    {
        block_on(run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb));
    }
}
