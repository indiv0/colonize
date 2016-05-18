#version 140

in vec2 position;
in vec2 tex_coord;

out vec2 uv;

uniform sampler2D tex;
uniform mat4 matrix;

void main() {
    vec2 size = vec2(textureSize(tex, 0));
    uv = vec2(tex_coord) / size;
    uv.y = 1.0 - uv.y;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}
