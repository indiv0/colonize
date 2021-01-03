#version 300 es
#define WEBGL

precision highp float;

in vec3 Vertex_Position;
in vec3 Vertex_Normal;
#ifdef STANDARDMATERIAL_ALBEDO_TEXTURE
in vec2 Vertex_Uv;
#endif

out vec3 v_Position;
out vec3 v_Normal;
out vec2 v_Uv;

layout(std140) uniform Camera {
    mat4 ViewProj;
};

layout(std140) uniform Transform {  // set = 2, binding = 0
    mat4 Model;
};

void main() {
    v_Normal = (Model * vec4(Vertex_Normal, 1.0)).xyz;
    v_Normal = mat3(Model) * Vertex_Normal;
    v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    #ifdef STANDARDMATERIAL_ALBEDO_TEXTURE
    v_Uv = Vertex_Uv;
    #endif
    gl_Position = ViewProj * vec4(v_Position, 1.0);
}