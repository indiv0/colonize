#version 310 es

layout(location = 0) out mediump vec4 o_Target;

layout(location = 0) in mediump vec3 Vertex_Position;
layout(location = 1) in mediump float vertex_y;
layout(location = 2) in mediump float y_level;

layout(set = 2, binding = 0) uniform MeshMaterial_color {
    mediump vec4 color;
};

layout(set = 3, binding = 0) uniform Time {
    mediump float TimeElapsed;
};

void main() {
    o_Target = color * 1.0;
}