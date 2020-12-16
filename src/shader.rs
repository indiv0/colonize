use bevy::{ecs::{Commands, Res, ResMut}, prelude::{AddAsset, AppBuilder, AssetServer, Assets, Handle, Mesh, Plugin, RenderPipelines, Shader, Texture}, reflect::TypeUuid, render::{pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, base}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};

//const VERTEX_SHADER: &str = r#"
//#version 300es
//#define STANDARDMATERIAL_SHADED
//#derive WEBGL
//precision highp float;
//in vec3 Vertex_Position;
//in vec3 Vertex_Normal;
//in vec3 Vertex_Uv;
//out vec3 v_Position;
//out vec3 v_Normal;
//out vec2 v_Uv;
//layout(std140) uniform Camera {
//    mat4 ViewProj;
//}
//layout(std140) uniform Transform {
//    mat4 Model;
//}
//
//void main() {
//    v_Normal = (Model * vec4(Vertex_Normal, 1.0)).xyz;
//    v_Normal = mat4(Model) * Vertex_Normal;
//    v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
//    v_Uv = Vertex_Uv;
//    gl_Position = ViewProj * vec4(v_Position, 1.0);
//}
//"#;
//
//const FRAGMENT_SHADER: &str = r#"
//#version 300 es
//#define STANDARDMATERIAL_SHADED
//#define WEBGL
//
//precision highp float;
//
//const int MAX_LIGHTS = 10;
//
//struct Light {
//    mat4 proj;
//    vec4 pos;
//    vec4 color;
//};
//
//in vec3 v_Position;
//in vec3 v_Normal;
//in vec2 v_Uv;
//
//out vec4 o_Target;
//
//layout(std140) uniform Camera {
//    mat4 ViewProj;
//};
//
//layout(std140) uniform Lights {  // set = 1, binding = 0
//    vec3 AmbientColor;
//    uvec4 NumLights;
//    Light SceneLights[MAX_LIGHTS];
//};
//
//layout(std140) uniform StandardMaterial_albedo { // set = 3, binding = 0
//    vec4 Albedo;
//};
//
//#ifdef STANDARDMATERIAL_ALBEDO_TEXTURE
//uniform sampler2D StandardMaterial_albedo_texture;  // set = 3, binding = 1
//#endif
//
//vec4 encodeSRGB(vec4 linearRGB_in)
//{
//    vec3 linearRGB = linearRGB_in.rgb;
//    vec3 a = 12.92 * linearRGB;
//    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
//    vec3 c = step(vec3(0.0031308), linearRGB);
//    return vec4(mix(a, b, c), linearRGB_in.a);
//}
//
//void main() {
//    vec4 output_color = Albedo;
//    
//    #ifdef STANDARDMATERIAL_ALBEDO_TEXTURE
//    output_color *= texture(
//        StandardMaterial_albedo_texture,
//        v_Uv
//    );
//    #endif
//    
//    #ifdef STANDARDMATERIAL_SHADED
//    vec3 normal = normalize(v_Normal);
//    // accumulate color
//    vec3 color = AmbientColor;
//    for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
//        Light light = SceneLights[i];
//        // compute Lambertian diffuse term
//        vec3 light_dir = normalize(light.pos.xyz - v_Position);
//        float diffuse = max(0.0, dot(normal, light_dir));
//        // add light contribution
//        color += diffuse * light.color.xyz;
//    }
//    output_color.xyz *= color;
//    #endif
//    // multiply the light by material color
//    o_Target = encodeSRGB(output_color);
//    // o_Target = output_color;
//}
//"#;

const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 3) in float Vertex_Voxel;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;
layout(location = 3) out float v_Voxel;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Normal = (Model * vec4(Vertex_Normal, 1.0)).xyz;
    v_Normal = mat3(Model) * Vertex_Normal;
    v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    v_Uv = Vertex_Uv;
    v_Voxel = Vertex_Voxel;
    gl_Position = ViewProj * vec4(v_Position, 1.0);
    //v_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    //gl_Position = v_Position;
}
"#;

const FRAGMENT_SHADER: &str = r#"
//#version 450
//
//layout(location = 0) in vec4 v_Position;
//layout(location = 0) out vec4 o_Target;
//
//layout(set = 2, binding = 0) uniform texture2DArray ArrayTexture_texture;
//layout(set = 2, binding = 1) uniform sampler ArrayTexture_texture_sampler;
//
//void main() {
//    // Screen-space coordinates determine which layer of the array texture we sample.
//    vec2 ss = v_Position.xy / v_Position.w;
//    float layer = 0.0;
//    if (ss.x > 0.0 && ss.y > 0.0) {
//        layer = 0.0;
//    } else if (ss.x < 0.0 && ss.y > 0.0) {
//        layer = 1.0;
//    } else if (ss.x > 0.0 && ss.y < 0.0) {
//        layer = 2.0;
//    } else {
//        layer = 3.0;
//    }
//    layer = 3.0;
//
//    // Convert to texture coordinates.
//    vec2 uv = (ss + vec2(1.0)) / 2.0;
//
//    o_Target = vec4(1.0, 1.0, 1.0, 0.5);
//    //o_Target = texture(sampler2DArray(ArrayTexture_texture, ArrayTexture_texture_sampler), vec3(uv, layer));
//}

#version 450

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;
layout(location = 3) in float v_Voxel;
//layout(location = 4) in float v_Vox_Val;
//layout(location = 6) in float v_AO;
layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

//layout(set = 1, binding = 0) uniform Lights {
//    vec3 AmbientColor;
//    uvec4 NumLights;
//    Light SceneLights[MAX_LIGHTS];
//};

//layout(set = 3, binding = 0) uniform MyMaterial_albedo {
//    vec4 Albedo;
//};

//layout(set = 2, binding = 0) uniform texture2DArray MyMaterial_albedo_texture;
//layout(set = 3, binding = 2) uniform sampler MyMaterial_albedo_texture_sampler;

//layout(set = 3, binding = 3) uniform MyMaterial_custom_val {
//    float custom_val;
//};

void main() {
    //vec4 output_color = Albedo;
    //// output_color *= texture(
    ////     sampler2DArray(MyMaterial_albedo_texture, MyMaterial_albedo_texture_sampler),
    ////     vec3(v_Uv, v_Vox_Val));

    //output_color *= vec4(vec3(v_Vox_Val), 1.0);

    //vec3 normal = normalize(v_Normal);
    //// accumulate color
    //float ao = 3.0 - v_AO;
    //vec3 color = AmbientColor - vec3(ao/50.0);
    //for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
    //    Light light = SceneLights[i];
    //    // compute Lambertian diffuse term
    //    vec3 light_dir = normalize(light.pos.xyz - v_Position);
    //    float diffuse = max(0.0, dot(normal, light_dir));
    //    // add light contribution
    //    color += diffuse * light.color.xyz;
    //}
    //output_color.xyz *= color;

    //// multiply the light by material color
    //o_Target = output_color;
    float output_color = v_Voxel / 2.0;
    o_Target = vec4(output_color, output_color, output_color, 1.0);
}
"#;

pub(crate) struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_asset::<ArrayTexture>()
            .add_startup_system(setup)
            .add_system(create_array_texture);
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "5796664a-8eaa-484f-b326-d54d1880185f"]
pub(crate) struct ArrayTexture {
    texture: Handle<Texture>,
}

struct LoadingTexture(Option<Handle<Texture>>);

pub(crate) struct ArrayTexturePipeline(pub(crate) Handle<PipelineDescriptor>);

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Start loading the texture.
    commands.insert_resource(LoadingTexture(Some(
        asset_server.load("terrain.png"),
    )));

    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));
    commands.insert_resource(ArrayTexturePipeline(pipeline_handle));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind ArrayTexture resources to our shader.
    render_graph.add_system_node(
        "array_texture",
        AssetRenderResourcesNode::<ArrayTexture>::new(true),
    );
    // Add a Render Graph edge connecting our new "array_texture" node to the main pass node. This ensures "my_array_texture"
    // runs before the main pass.
    render_graph
        .add_node_edge("array_texture", base::node::MAIN_PASS)
        .unwrap();

    //commands.spawn(Camera3dBundle {
    //    transform: Transform::from_translation(Vec3::new(2.0, 2.0, 2.0))
    //        .looking_at(Vec3::default(), Vec3::unit_y()),
    //    ..Default::default()
    //});
}

fn create_array_texture(
    commands: &mut Commands,
    pipeline: Res<ArrayTexturePipeline>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut array_textures: ResMut<Assets<ArrayTexture>>,
) {
    let (handle, texture) = match loading_texture.0.as_ref() {
        Some(handle) => {
            if let Some(texture) = textures.get_mut(handle) {
                (loading_texture.0.take().unwrap(), texture)
            } else {
                return;
            }
        }
        None => return,
    };

    // Create a new array texture asset from the loaded texture.
    let array_layers = 5;
    texture.reinterpret_stacked_2d_as_array(array_layers);
    let array_texture = array_textures.add(ArrayTexture { texture: handle });

    //// Spawn a cube that's shaded using the array texture.
    //commands
    //    .spawn(MeshBundle {
    //        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
    //            pipeline.0.clone(),
    //        )]),
    //        ..Default::default()
    //    })
    //    .with(array_texture);
}