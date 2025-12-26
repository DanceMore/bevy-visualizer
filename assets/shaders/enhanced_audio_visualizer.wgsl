// Enhanced audio-reactive shader with vibrant colors and complex patterns
#import bevy_pbr::forward_io::VertexOutput

// Include some utility functions for color conversion and noise
fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3(p.xyx) * 0.1031);
    p3 = p3 + dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise(p: vec2<f32>) -> f32 {
    let p_floor = floor(p);
    let f = fract(p);
    
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(mix(hash(p_floor + vec2<f32>(0.0, 0.0)),
                   hash(p_floor + vec2<f32>(1.0, 0.0)), u.x),
               mix(hash(p_floor + vec2<f32>(0.0, 1.0)),
                   hash(p_floor + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

// Convert RGB to HSV
fn rgb2hsv(rgb: vec3<f32>) -> vec3<f32> {
    let cmax = max(rgb.r, max(rgb.g, rgb.b));
    let cmin = min(rgb.r, min(rgb.g, rgb.b));
    let delta = cmax - cmin;
    
    // Calculate hue
    var h = 0.0;
    if (delta != 0.0) {
        if (rgb.r == cmax) {
            h = 60.0 * (((rgb.g - rgb.b) / delta) % 6.0);
        } else if (rgb.g == cmax) {
            h = 60.0 * (((rgb.b - rgb.r) / delta) + 2.0);
        } else {
            h = 60.0 * (((rgb.r - rgb.g) / delta) + 4.0);
        }
    }
    
    // Calculate saturation
    var s = 0.0;
    if (cmax != 0.0) {
        s = delta / cmax;
    }
    
    let v = cmax;
    
    return vec3(h, s, v);
}

// Convert HSV to RGB
fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    let c = hsv.z * hsv.y;
    let x = c * (1.0 - abs((hsv.x / 60.0) % 2.0 - 1.0));
    let m = hsv.z - c;
    
    if (hsv.x >= 0.0 && hsv.x < 60.0) {
        return vec3(c, x, 0.0) + m;
    } else if (hsv.x >= 60.0 && hsv.x < 120.0) {
        return vec3(x, c, 0.0) + m;
    } else if (hsv.x >= 120.0 && hsv.x < 180.0) {
        return vec3(0.0, c, x) + m;
    } else if (hsv.x >= 180.0 && hsv.x < 240.0) {
        return vec3(0.0, x, c) + m;
    } else if (hsv.x >= 240.0 && hsv.x < 300.0) {
        return vec3(x, 0.0, c) + m;
    } else {
        return vec3(c, 0.0, x) + m;
    }
}

// Fractional Brownian Motion (fBm) for more complex noise
fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 0.0;
    
    for (var i = 0; i < 6; i++) {
        value += amplitude * noise(p * pow(2.0, f32(i)));
        amplitude *= 0.5;
    }
    
    return value;
}

struct UShaderData {
    r: f32,  // Bass frequency amplitude (20-250Hz)
    g: f32,  // Midrange frequency amplitude (250-4000Hz)
    b: f32,  // Treble frequency amplitude (4000-20000Hz)
    time: f32, // Time for animation
};

@group(3) @binding(0) var<uniform> shader_data: UShaderData;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Get audio data (already processed and normalized in Rust code)
    let bass = shader_data.r;      // 20-250Hz
    let mid = shader_data.g;      // 250-4000Hz
    let treble = shader_data.b;   // 4000-20000Hz
    let time = shader_data.time;
    
    // Calculate overall intensity with more dynamic range
    let total_intensity = bass + mid + treble;
    let max_intensity = max(max(bass, mid), treble);
    
    // Get UV coordinates
    let uv = in.uv;
    
    // Create a radial coordinate system from center
    let center = vec2<f32>(0.5, 0.5);
    let distance_from_center = distance(uv, center);
    let angle = atan2(uv.y - center.y, uv.x - center.x);
    
    // Enhanced color generation using HSV color space
    // Map frequency dominance to specific hue ranges for strong primary colors
    let hue = select(
        select(
            mix(0.0, 60.0, bass),    // Red to Yellow (bass dominant)
            mix(120.0, 180.0, mid),  // Green to Cyan (mid dominant)
            mid > bass && mid > treble
        ),
        mix(240.0, 300.0, treble), // Blue to Magenta (treble dominant)
        treble > bass && treble > mid
    );
    
    // High saturation for vibrant colors
    let saturation = mix(0.7, 1.0, total_intensity);
    
    // Value (brightness) that pulses with audio intensity
    let value = 0.5 + 0.5 * pow(sin(time * 2.0 + total_intensity * 5.0), 2.0);
    
    // Base color in HSV, then convert to RGB
    let base_color = hsv2rgb(vec3(hue, saturation, value));
    
    // Create complex audio-reactive patterns
    // 1. Bass creates concentric circles with distortion
    let bass_circles = sin(distance_from_center * 10.0 - time * 2.0 + bass * 15.0) * 0.3;
    
    // 2. Mid creates radial waves
    let mid_waves = sin(angle * 8.0 + time * 3.0 + mid * 20.0) * 0.2;
    
    // 3. Treble creates high-frequency noise patterns
    let treble_noise = fbm(uv * 20.0 + vec2(time * 5.0, treble * 30.0)) * 0.4;
    
    // 4. Audio-driven cellular patterns
    let cellular = noise(uv * 50.0 + vec2(time * 3.0, total_intensity * 10.0));
    
    // Combine patterns with different weights based on frequency dominance
    let pattern_intensity = bass_circles + mid_waves + treble_noise + cellular * 0.5;
    
    // Create color variations based on patterns
    let pattern_color = hsv2rgb(vec3(
        hue + pattern_intensity * 60.0,  // Shift hue based on patterns
        saturation * 1.2,                // Boost saturation in patterns
        value * (1.0 + pattern_intensity) // Brighten patterns
    ));
    
    // Mix base color with pattern color
    let mixed_color = mix(base_color, pattern_color, abs(pattern_intensity) * 0.7);
    
    // Add radial gradient for depth
    let radial_gradient = 1.0 - distance_from_center * 0.7;
    
    // Final color with enhanced contrast
    let final_color = mixed_color * radial_gradient * (1.0 + total_intensity * 2.0);
    
    // Ensure strong color output by boosting saturation in final step
    let final_hsv = rgb2hsv(final_color);
    let boosted_color = hsv2rgb(vec3(
        final_hsv.x,
        min(final_hsv.y * 1.5, 1.0),  // Boost saturation
        min(final_hsv.z * 1.2, 1.0)   // Boost value
    ));
    
    return vec4<f32>(boosted_color, 1.0);
}