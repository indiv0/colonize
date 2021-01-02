#version 310 es

layout(location = 0) in mediump vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    if (Vertex_Position.y >= 0.5) {
        gl_Position = vec4(vec3(2.), 1.0);
    } else {
        gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    }
}