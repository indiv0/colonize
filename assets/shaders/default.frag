#version 300 es
#define WEBGL

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

layout(std140) uniform Lights {// set = 1, binding = 0
    vec3 AmbientColor;
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(std140) uniform StandardMaterial_albedo { // set = 3, binding = 0 
    vec4 Albedo;
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
}