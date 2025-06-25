#version 320 es
// Cyber Green Gradient Tint Shader (Border-Matched)
// Creates subtle gradient tinting that matches border colors

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 TINT_COLOR = vec3(0.0, 1.0, 0.55);  // Cyber green
const vec3 ACCENT_COLOR = vec3(0.0, 1.0, 0.8); // Electric cyan accent

vec3 apply_gradient_tint(vec3 original, vec2 coord) {
    // Create subtle radial gradient from center with slight pixelation
    float pixel_size = 64.0; // Very subtle pixelation
    vec2 pixelated_coord = floor(coord * pixel_size) / pixel_size;
    float center_distance = length(pixelated_coord - vec2(0.5, 0.5));
    float gradient_strength = smoothstep(0.0, 0.8, center_distance);
    
    // Preserve luminance while adding color tint (slightly more vibrant)
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    vec3 tinted = mix(original, TINT_COLOR * (0.75 + luminance * 0.65), 0.15 * gradient_strength);
    
    // Add subtle edge glow with accent color
    float edge_glow = 1.0 - smoothstep(0.0, 0.4, center_distance);
    tinted += mix(TINT_COLOR, ACCENT_COLOR, 0.3) * edge_glow * 0.06;
    
    // Add coding-specific matrix effect in corners
    float corner_effect = max(
        max(abs(coord.x - 0.5), abs(coord.y - 0.5)) - 0.4,
        0.0
    ) * 10.0;
    tinted += TINT_COLOR * corner_effect * 0.025;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_gradient_tint(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
