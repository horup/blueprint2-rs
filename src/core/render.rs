use glow::*;

pub struct Render {
    pub gl:Context,
    pub width:i32,
    pub height:i32
}

impl Render {
    unsafe fn setup_shaders(&mut self) {
        let gl = &mut self.gl;
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
    }

    unsafe fn setup_buffers(&mut self) {
        let vertex_array = self.gl.create_vertex_array().expect("cannot create vertex array");
        self.gl.bind_vertex_array(Some(vertex_array));
    }

    pub fn new(gl:Context) -> Self {
        let mut render = Render {
            gl:gl,
            width:1,
            height:1
        };
        unsafe {
            render.setup_buffers();
            render.setup_shaders();
        }
        return render;
    }
    
    pub fn draw(&mut self) {
        let mut gl = &mut self.gl;
        unsafe {
            gl.viewport(0, 0, self.width, self.height);
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}