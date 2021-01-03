#version 310 es

layout(location = 0) in mediump vec3 Vertex_Position;
layout(location = 1) out mediump float vertex_y;
layout(location = 2) out mediump float y_level;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 3, binding = 0) uniform Time {
    mediump float TimeElapsed;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}