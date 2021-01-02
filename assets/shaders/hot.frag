#version 310 es

layout(location = 0) out mediump vec4 o_Target;

layout(set = 2, binding = 0) uniform MeshMaterial_color {
    mediump vec4 color;
};

void main() {
    o_Target = color * 1.0;
}