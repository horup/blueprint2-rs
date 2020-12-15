use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use glow::*;

// TODO: add console out functionality


#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();


    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let (gl, render_loop) = (
        glow::Context::from_webgl2_context(webgl2_context),
        glow::RenderLoop::from_request_animation_frame()
    );

    unsafe {

        let vertex_shader_source = include_str!("./shaders/default.vert");
        let fragment_shader_source = include_str!("./shaders/default.frag");

        let program = gl.create_program().expect("Cannot create program");
    
        let shader = gl.create_shader(glow::VERTEX_SHADER).expect("could not create shader");
        gl.shader_source(shader, vertex_shader_source);
        gl.compile_shader(shader);
        gl.attach_shader(program, shader);

        let shader = gl.create_shader(glow::FRAGMENT_SHADER).expect("could not create shader");
        gl.shader_source(shader, fragment_shader_source);
        gl.compile_shader(shader);
        gl.attach_shader(program, shader);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }

        gl.use_program(Some(program));

        render_loop.run(move |running| {
                gl.clear_color(1.0, 0.0, 0.0, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
        }); 
    }
}

//https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs