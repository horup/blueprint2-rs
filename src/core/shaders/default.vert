#version 300 es
layout (location = 0) in vec3 pos; 
layout (location = 1) in vec2 uv; 

out vec2 tex_coord;

void main() {
    tex_coord = uv;
    gl_Position = vec4(pos, 1.0);
}