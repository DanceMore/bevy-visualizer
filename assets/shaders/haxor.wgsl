// Simple and effective audio-reactive shader
#import bevy_pbr::forward_io::VertexOutput

struct UShaderData {
    r: f32,  // Bass frequency amplitude (20-250Hz)
    g: f32,  // Midrange frequency amplitude (250-4000Hz)  
    b: f32,  // Treble frequency amplitude (4000-20000Hz)
    _pad: f32,
};

@group(3) @binding(0) var<uniform> shader_data: UShaderData;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Get audio data (already processed and normalized in Rust code)
    let bass = shader_data.r;      // 20-250Hz
    let mid = shader_data.g;      // 250-4000Hz
    let treble = shader_data.b;   // 4000-20000Hz
    
    // Calculate overall intensity
    let total_intensity = bass + mid + treble;
    
    // Get UV coordinates
    let uv = in.uv;
    
    // Create a simple radial gradient from center
    let center = vec2<f32>(0.5, 0.5);
    let distance_from_center = distance(uv, center);
    
    // Audio-reactive color based on frequency distribution
    // Bass dominates: Red/Orange
    // Mid dominates: Green/Cyan  
    // Treble dominates: Blue/Purple
    let red = mix(0.1, 1.0, bass);
    let green = mix(0.1, 1.0, mid);
    let blue = mix(0.1, 1.0, treble);
    
    // Make the color pulse with overall intensity
    let pulse = 0.5 + 0.5 * sin(total_intensity * 10.0);
    
    // Create audio-reactive patterns
    // Bass creates low-frequency waves
    let bass_wave = sin(uv.x * 2.0 + bass * 5.0) * sin(uv.y * 2.0 + bass * 5.0) * 0.2;
    
    // Mid creates medium-frequency patterns
    let mid_pattern = sin(uv.x * 5.0 + mid * 10.0) * sin(uv.y * 5.0 + mid * 10.0) * 0.3;
    
    // Treble creates high-frequency noise
    let treble_noise = sin(uv.x * 20.0 + treble * 20.0) * sin(uv.y * 20.0 + treble * 20.0) * 0.1;
    
    // Combine all effects
    let intensity = (1.0 + bass_wave + mid_pattern + treble_noise) * pulse;
    
    // Final color with intensity modulation
    let final_color = vec3<f32>(red, green, blue) * intensity;
    
    // Ensure the center is brighter when there's more audio activity
    let center_boost = 1.0 - distance_from_center * 0.5;
    let final_color_boosted = final_color * (1.0 + center_boost * total_intensity);
    
    return vec4<f32>(final_color_boosted, 1.0);
}
