use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod note;
mod renderer;

use renderer::Renderer;

use futures::executor::block_on;

async fn run() {
    println!("Started running.");

    let event_loop = EventLoop::new();

    let window = Window::new(&event_loop).unwrap();
    let mut renderer: Renderer = Renderer::new(&window).await;

    event_loop.run(
        move |event: Event<()>, _, controlflow: &mut ControlFlow| match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => *controlflow = ControlFlow::Exit,

                WindowEvent::Resized(size) => {
                    renderer.resize(*size);
                }

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

            Event::RedrawRequested(_) => {
                renderer.render();
                renderer.update();
            }

            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        },
    );
}

fn select_wasm_or_native() {
    #[cfg(feature = "native")]
    {
        env_logger::init();

        block_on(run());
    }
    #[cfg(feature = "wasm")]
    {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Trace).unwrap();

        futures::executor::block_on(run());
    }
}

fn main() {
    select_wasm_or_native();
}
