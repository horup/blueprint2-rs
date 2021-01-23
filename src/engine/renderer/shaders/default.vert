#version 300 es
layout (location = 0) in vec3 pos; 
layout (location = 1) in vec2 uv; 

out vec2 tex_coord;

uniform mat4 view;
uniform mat4 projection;

void main() {
    tex_coord = uv;

    gl_Position =  view * projection * vec4(pos, 1.0);
}