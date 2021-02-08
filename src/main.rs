
use core::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use glow::*;
use web_sys::{CssStyleDeclaration, HtmlCanvasElement};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

use crate::{game::{BlueprintGame, Engine}};

//use crate::engine::*;
//use crate::game::*;


// TODO: add console out instead of stack exception to improve debugability
// TODO: add verted buffer to store vertices for triangle drawing
// TODO: spin the triangle
// TODO: measure FPS
// TODO: add timing
// TODO: add texture support


#[wasm_bindgen(start)]
pub fn start() {
    let event_loop = EventLoop::new();
    use winit::platform::web::WindowExtWebSys;
    let mut window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    {
        
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let canvas:HtmlCanvasElement = window.canvas();
        canvas.style().set_css_text("");
        body.append_child(&canvas);

        let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();


        let gl = glow::Context::from_webgl2_context(webgl2_context);
        let mut engine:Engine<BlueprintGame> = Engine::new(gl);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    engine.renderer.width = canvas.width() as i32;
                    engine.renderer.height = canvas.height() as i32;
                    engine.update(&mut window);
                    window.request_redraw();
                }
                _ => (),
            }
        });

    }
}

pub fn main() {
    // TODO: implement native
}


//https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs
//https://webglfundamentals.org/webgl/lessons/webgl-how-it-works.html