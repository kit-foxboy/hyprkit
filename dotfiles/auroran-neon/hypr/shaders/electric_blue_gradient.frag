#version 320 es
// Neon Marquee Shader - "Blade Runner Ambiance"
// Atmospheric neon lighting with cinematic glow (GPU optimized)

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 NEON_BLUE = vec3(0.0, 0.7, 1.0);      // Main neon blue
const vec3 ELECTRIC_CYAN = vec3(0.0, 1.0, 0.9);  // Bright accent
const vec3 DEEP_BLUE = vec3(0.0, 0.4, 0.8);      // Shadow blue

vec3 apply_neon_marquee_effect(vec3 original, vec2 coord) {
    // Static atmospheric depth gradient
    float center_distance = length(coord - vec2(0.5, 0.5));
    float atmosphere = 1.0 - smoothstep(0.0, 1.2, center_distance);
    
    // Neon tube glow effect around edges
    float edge_distance = min(min(coord.x, 1.0 - coord.x), min(coord.y, 1.0 - coord.y));
    float neon_glow = exp(-edge_distance * 8.0);
    
    // Static electric pattern (simulates shimmer without animation)
    float pattern = sin(coord.x * 40.0) * sin(coord.y * 30.0) * 0.5 + 0.5;
    pattern = pow(pattern, 2.0); // Make it more focused
    
    // Cinematic haze overlay
    float haze = 1.0 - center_distance * 0.6;
    
    // Preserve original luminance
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    
    // Base atmospheric blue tint
    vec3 tinted = mix(original, NEON_BLUE * (0.8 + luminance * 0.5), 0.12 * atmosphere);
    
    // Add neon marquee edge glow
    tinted += ELECTRIC_CYAN * neon_glow * 0.15;
    
    // Add cinematic depth with deeper blue in center
    tinted += DEEP_BLUE * (1.0 - atmosphere) * 0.08;
    
    // Add static electric pattern
    tinted += ELECTRIC_CYAN * pattern * neon_glow * 0.02;
    
    // Apply overall cinematic haze
    tinted *= (0.9 + haze * 0.1);
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_neon_marquee_effect(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
