// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct UShaderData {
    r: f32,
    g: f32,
    b: f32,
    _pad: f32,
};

@group(1) @binding(0) var<uniform> shader_data: UShaderData;

fn oklab_to_linear_srgb(c: vec3<f32>) -> vec3<f32> {
    let L = c.x;
    let a = c.y;
    let b = c.z;

    let l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    return vec3<f32>(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}

//@fragment
//fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
//    let speed = 2.0;
//    // The globals binding contains various global values like time
//    // which is the time since startup in seconds
//    let t_1 = sin(globals.time * speed) * 0.5 + 0.5;
//    let t_2 = cos(globals.time * speed);
//
//    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) * 1.4;
//
//    // blending is done in a perceptual color space: https://bottosson.github.io/posts/oklab/
//    let red = vec3<f32>(0.627955, 0.224863, 0.125846);
//    let green = vec3<f32>(0.86644, -0.233887, 0.179498);
//    let blue = vec3<f32>(0.701674, 0.274566, -0.169156);
//
//    //let red =   vec3<f32>(shader_data.r, 0.0, 0.0);
//    //let green = vec3<f32>(0.0, shader_data.g, 0.0);
//    //let blue =  vec3<f32>(0.0, 0.0, shader_data.b);
//    let white = vec3<f32>(1.0, 0.0, 0.0);
//    let mixed = mix(mix(red, blue, t_1), mix(green, white, t_2), distance_to_center);
//    //let mixed = mix(mix(red, blue, shader_data.r), mix(green, white, shader_data.r), distance_to_center);
//
//    //return vec4<f32>(oklab_to_linear_srgb(mixed), 0.0);
//    return vec4<f32>(shader_data.r, shader_data.g, shader_data.b, 0.0);
//    //return vec4<f32>(0.0, 0.0, 1.0, 0.0);
//    //return vec4<f32>(0.1, 0.0, 0.0, 0.0);
//    //return vec4<f32>(shader_data_r, shader_data_g, shader_data_b, 0.0);
//}


//@fragment
//fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
//    let audioValue = shader_data.r; // Sample audio data
//
//    // Create a dynamic wave pattern based on audio amplitude
//    let frequency = 2.0 + audioValue * 10.0; // Adjust the frequency of the waves based on audioValue
//    let time = 10.0 * f32(globals.time);
//    let wave = 0.5 + 0.5 * f32(sin(frequency * time)); // Dynamic wave pattern
//
//    // Use the frequency to set colors
//    let red = 0.5 + 0.5 * f32(sin(frequency * 2.0));
//    let green = 0.5 + 0.5 * f32(cos(frequency * 3.0));
//    let blue = 0.5 + 0.5 * f32(sin(frequency * 1.5));
//
//    return vec4<f32>(red, green, blue, 1.0);
//}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let audioValue = shader_data.r; // Sample audio data

    let scaled_value = audioValue * 1000.0;

    let red = f32(sin(scaled_value));
    let green = 0.0;
    let blue = 0.2;

    return vec4<f32>(red, green, blue, 1.0);
}
