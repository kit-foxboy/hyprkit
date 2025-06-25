#version 320 es
// Cyber Orange Gradient Tint Shader (Border-Matched)
// Creates subtle gradient tinting that matches border colors

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 TINT_COLOR = vec3(1.0, 0.4, 0.0);     // More vibrant cyber orange
const vec3 ACCENT_COLOR = vec3(1.0, 0.6, 0.0);  // Bright orange accent
const vec3 HOT_COLOR = vec3(1.0, 0.2, 0.0);     // Hot red-orange

vec3 apply_gradient_tint(vec3 original, vec2 coord) {
    // Create dynamic radial gradient from center
    float center_distance = length(coord - vec2(0.5, 0.5));
    float gradient_strength = smoothstep(0.0, 0.9, center_distance);
    
    // Preserve luminance while adding intense color tint
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    vec3 tinted = mix(original, TINT_COLOR * (0.8 + luminance * 0.7), 0.25 * gradient_strength);
    
    // Add vibrant edge glow with hot accent
    float edge_glow = 1.0 - smoothstep(0.0, 0.3, center_distance);
    tinted += mix(ACCENT_COLOR, HOT_COLOR, 0.5) * edge_glow * 0.15;
    
    // Add intense media-specific energy waves
    float wave_effect = sin(coord.y * 12.0) * cos(coord.x * 10.0) * 0.5 + 0.5;
    float pulse = sin(coord.x * 15.0 + coord.y * 15.0) * 0.5 + 0.5;
    tinted += TINT_COLOR * wave_effect * gradient_strength * 0.08;
    tinted += HOT_COLOR * pulse * edge_glow * 0.06;
    
    // Add corner intensity boosts
    float corner_boost = max(max(coord.x, 1.0 - coord.x), max(coord.y, 1.0 - coord.y));
    corner_boost = smoothstep(0.7, 1.0, corner_boost);
    tinted += ACCENT_COLOR * corner_boost * 0.1;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_gradient_tint(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
