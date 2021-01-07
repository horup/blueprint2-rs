
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use glow::*;

use crate::{Engine, Event, Game, World, log};


// TODO: add console out instead of stack exception to improve debugability
// TODO: add verted buffer to store vertices for triangle drawing
// TODO: spin the triangle
// TODO: measure FPS
// TODO: add timing
// TODO: add texture support


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


    let mut engine = Engine::new(gl);
    let mut game = Game::new();

    engine.update(Event::Initialize, &mut game);
    render_loop.run(move |running| { 
        engine.width = canvas.width() as i32;
        engine.height = canvas.height() as i32;
        engine.update(Event::Update(1.0), &mut game);
        engine.update(Event::Draw(1.0), &mut game);
    });


 /*   let mut render = Render::new(gl);
  
    let q1 = render.insert_quad();
    let q2 = render.insert_quad();

    let m = render.get_mesh_mut(q2).unwrap();
    for v in &mut m.vertices {
        v.x += 0.5;
        v.y += 0.5;
    }
    let m = render.get_mesh(q2).unwrap();
    m.update(&render.gl);

    
    let mut world = World::new();
    update(Context {
        world:&mut world,
        event:Event::Initialize
    });

    render_loop.run(move |running| {
        render.width = canvas.width() as i32;
        render.height = canvas.height() as i32;

        update(Context {
            world:&mut world,
            event:Event::Update(1.0)
        });

        render.draw();
    });*/
}


//https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs
//https://webglfundamentals.org/webgl/lessons/webgl-how-it-works.html