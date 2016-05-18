#version 140

in vec2 uv;

out vec4 out_color;

uniform sampler2D tex;
uniform vec4 color_multiply;

void main() {
    out_color = color_multiply * texture(tex, uv);
}
