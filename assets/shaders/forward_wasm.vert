#version 300 es
// MIT License
// 
// Copyright (c) 2020 Mariusz Kryński
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
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
