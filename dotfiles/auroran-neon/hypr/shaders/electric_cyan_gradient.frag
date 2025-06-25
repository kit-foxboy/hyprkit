#version 320 es
// Electric Cyan Gradient - "Terminal Glow"
// Terminal personality: Clean cyan tinting with subtle digital ambiance

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 ELECTRIC_CYAN = vec3(0.0, 1.0, 1.0);    // Primary cyan
const vec3 DEEP_TEAL = vec3(0.0, 0.8, 0.9);        // Darker accent
const vec3 BRIGHT_AQUA = vec3(0.3, 1.0, 1.0);      // Light highlight

vec3 apply_cyan_gradient_effect(vec3 original, vec2 coord) {
    // Subtle vertical gradient for terminal-like feel
    float vertical_gradient = coord.y * 0.8 + 0.2;
    
    // Gentle edge highlighting
    float edge_distance = min(min(coord.x, 1.0 - coord.x), min(coord.y, 1.0 - coord.y));
    float edge_glow = 1.0 - smoothstep(0.0, 0.4, edge_distance);
    edge_glow = pow(edge_glow, 2.0) * 0.1; // Soft falloff
    
    // Preserve original luminance
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    
    // Base cyan tint with vertical gradient
    vec3 tinted = mix(original, ELECTRIC_CYAN * (0.7 + luminance * 0.3), 0.06 * vertical_gradient);
    
    // Add subtle edge glow
    tinted += BRIGHT_AQUA * edge_glow;
    
    // Very subtle center-to-edge gradient for depth
    float center_distance = length(coord - vec2(0.5, 0.5));
    float center_gradient = 1.0 - smoothstep(0.0, 0.9, center_distance);
    tinted += DEEP_TEAL * center_gradient * 0.02;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_cyan_gradient_effect(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
