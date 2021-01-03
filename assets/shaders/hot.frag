#version 310 es

layout(location = 0) out mediump vec4 o_Target;
layout(location = 1) in mediump float vertex_y;

layout(set = 2, binding = 0) uniform MeshMaterial_color {
    mediump vec4 color;
};

layout(set = 3, binding = 0) uniform YLevel {
    mediump float YLevelValue;
};

void main() {
    if (vertex_y <= YLevelValue) {
        o_Target = color * 1.0;
    } else {
        discard;
    }
}

//#version 450
//
//layout(location = 0) out vec4 o_Target;
//
//layout(set = 2, binding = 0) uniform MeshMaterial_color {
//    vec4 color;
//};
//
//void main() {
//    o_Target = color * 1.0;
//}