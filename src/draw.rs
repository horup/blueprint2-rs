use crate::{Engine, World, log};
use glow::*;
pub fn setup_shaders(context:&mut Engine) {
    unsafe {

        let gl = &mut context.gl;
        let vertex_shader_source = include_str!("../shaders/default.vert");
        let fragment_shader_source = include_str!("../shaders/default.frag");

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

      /*  let pos_loc = gl.get_attrib_location(program, "pos").expect("get_attrib_location failed");
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, (std::mem::size_of::<u32>() * 3) as i32);
        gl.enable_vertex_attrib_array(1);*/

        gl.use_program(Some(program));
    }
}

pub fn draw(context:&mut Engine) {
    let gl = &context.gl;
    let width = context.width;
    let height = context.height;

    unsafe {
        gl.viewport(0, 0, width, height);
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        for (_, mesh) in &context.meshes {
            log("test");
            gl.bind_vertex_array(Some(mesh.vertex_array_object));
            gl.draw_arrays(glow::TRIANGLES, 0, mesh.vertices.len() as i32);
        }
    }
}