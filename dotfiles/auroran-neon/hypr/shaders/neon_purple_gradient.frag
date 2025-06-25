#version 320 es
// Neon Purple Gradient Tint Shader (Border-Matched)
// Creates subtle gradient tinting that matches border colors

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 TINT_COLOR = vec3(0.8, 0.0, 1.0);    // Neon purple
const vec3 ACCENT_COLOR = vec3(1.0, 0.0, 0.6);  // Neon pink accent

vec3 apply_gradient_tint(vec3 original, vec2 coord) {
    // Create subtle radial gradient from center
    float center_distance = length(coord - vec2(0.5, 0.5));
    float gradient_strength = smoothstep(0.0, 0.8, center_distance);
    
    // Preserve luminance while adding color tint
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    vec3 tinted = mix(original, TINT_COLOR * (0.7 + luminance * 0.6), 0.14 * gradient_strength);
    
    // Add subtle edge glow with accent color
    float edge_glow = 1.0 - smoothstep(0.0, 0.4, center_distance);
    tinted += mix(TINT_COLOR, ACCENT_COLOR, 0.4) * edge_glow * 0.06;
    
    // Add communication-specific mystical effect
    float mystical_effect = sin(coord.x * 8.0) * sin(coord.y * 6.0) * 0.5 + 0.5;
    tinted += TINT_COLOR * mystical_effect * gradient_strength * 0.02;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_gradient_tint(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}