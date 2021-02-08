#version 300 es
uniform sampler2D texture1;
precision mediump float;
in vec2 tex_coord;
out vec4 color;
void main() {
    color = texture(texture1, tex_coord);//(vec4(tex_coord.x, tex_coord.y, 0.0, 1.0);
}