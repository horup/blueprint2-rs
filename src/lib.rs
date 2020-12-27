mod render;
pub use render::*;

mod vertex;
pub use vertex::*;

mod mesh;
pub use mesh::*;

mod sprite;
pub use sprite::*;

mod world;
pub use world::*;

mod art;
pub use art::*;

mod math;
pub use math::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use glow::*;
use web_sys::console;

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


    let mut render = Render::new(gl);
   /* unsafe {
        let mesh = Mesh::new_quad(&mut render.gl);
        render.meshes.push(mesh);

        let mut mesh = Mesh::new_quad(&mut render.gl);

        for v in &mut mesh.vertices {
            v.x += 0.5;
            v.y += 0.5;
        }

        mesh.update(&mut render.gl);
        render.meshes.push(mesh);
    }*/

    let q1 = render.insert_quad();
    let q2 = render.insert_quad();

    let m = render.get_mesh_mut(q2).unwrap();
    for v in &mut m.vertices {
        v.x += 0.5;
        v.y += 0.5;
    }
    let m = render.get_mesh(q2).unwrap();
    m.update(&render.gl);

    render_loop.run(move |running| {
        render.width = canvas.width() as i32;
        render.height = canvas.height() as i32;
        render.draw();
    });
}


pub fn log(str:&str) {
    let array = js_sys::Array::new();
    array.push(&str.into());
    console::log(&array);
}

//https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs
//https://webglfundamentals.org/webgl/lessons/webgl-how-it-works.html