extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

fn get_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WebGlShader {
    let shader = context.create_shader(shader_type).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let compile_is_succeeded = context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap();
    if !compile_is_succeeded {
        panic!("Shader failed to compile");
    }

    return shader;
}

fn init_shaders(context: &WebGl2RenderingContext) -> WebGlProgram {
    let fragment_shader_file = include_str!("shader/arrow.fs");
    let vertex_shader_file = include_str!("shader/arrow.vs");

    let fragment_shader = get_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader_file);
    let vertex_shader = get_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader_file);

    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vertex_shader);
    context.attach_shader(&shader_program, &fragment_shader);
    context.link_program(&shader_program);

    let shader_is_created = context.get_program_parameter(&shader_program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap();

    if !shader_is_created {
        let info = context.get_program_info_log(&shader_program).unwrap();
        error(&format!("Shader creation error: {}", info));
    }

    return shader_program;
}

#[wasm_bindgen]
pub fn webgl2_sample(_canvas: HtmlCanvasElement, context: WebGl2RenderingContext) -> Result<(), JsValue>{

    let shader_program = init_shaders(&context);

    context.use_program(Some(&shader_program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 1.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}

#[wasm_bindgen]
pub fn start_game(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
    let width = canvas.width();
    let height = canvas.height();

    let divisions = 6;
    let square_width = width / divisions;
    let square_height = height / divisions;

    for x in 0..6 {
        for y in 0..6 {
            let color = &format!(
                "rgb({},{},{})",
                (255.0 - 20.5 * x as f64).floor(),
                (255.0 - 42.5 * y as f64).floor(),
                255,
            )
            .into();

            context.set_fill_style(color);
            log(&format!(
                "{:?} @ {}, {}, {}, {}",
                color,
                x * square_width,
                y * square_height,
                square_width,
                square_height
            ));
            context.fill_rect(
                f64::from(x * square_width),
                f64::from(y * square_height),
                f64::from(square_width),
                f64::from(square_height),
            );
        }
    }
}
