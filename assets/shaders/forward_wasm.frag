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

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

in vec3 v_Position;
in vec3 v_Normal;
in vec2 v_Uv;

out vec4 o_Target;

layout(std140) uniform Camera {
    mat4 ViewProj;
};

layout(std140) uniform Lights {  // set = 1, binding = 0
    vec3 AmbientColor;
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(std140) uniform StandardMaterial_albedo { // set = 3, binding = 0
    vec4 Albedo;
};

layout(std140) uniform YLevel { // set = 4, binding = 4
    vec3 YLevelValue;
};

#ifdef STANDARDMATERIAL_ALBEDO_TEXTURE
uniform sampler2D StandardMaterial_albedo_texture;  // set = 3, binding = 1
#endif

vec4 encodeSRGB(vec4 linearRGB_in)
{
    vec3 linearRGB = linearRGB_in.rgb;
    vec3 a = 12.92 * linearRGB;
    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
    vec3 c = step(vec3(0.0031308), linearRGB);
    return vec4(mix(a, b, c), linearRGB_in.a);
}

void main() {
    vec4 output_color = Albedo;

#ifdef STANDARDMATERIAL_ALBEDO_TEXTURE
    output_color *= texture(
        StandardMaterial_albedo_texture,
        v_Uv
    );
#endif

#ifdef STANDARDMATERIAL_SHADED
    vec3 normal = normalize(v_Normal);
    // accumulate color
    vec3 color = AmbientColor;
    for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        // compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += diffuse * light.color.xyz;
    }
    output_color.xyz *= color;
#endif
    // multiply the light by material color
    o_Target = encodeSRGB(output_color);
    // o_Target = output_color;

    // discard any fragments above the y-level
    if (v_Position.y > YLevelValue.x + 0.00001) {
        discard;
    }
}
