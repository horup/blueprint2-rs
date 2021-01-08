#version 300 es

precision mediump float;
in vec2 tex_coord;
out vec4 color;
void main() {
    color = vec4(tex_coord.x, tex_coord.y, 0.0, 1.0);
}