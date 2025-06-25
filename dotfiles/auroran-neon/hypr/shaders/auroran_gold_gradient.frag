#version 320 es
// Auroran Gold Gradient Tint Shader (Border-Matched)
// Creates subtle gradient tinting that matches border colors

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 TINT_COLOR = vec3(1.0, 0.8, 0.0);    // More vibrant auroran gold
const vec3 ACCENT_COLOR = vec3(1.0, 0.4, 0.0);  // Bright orange accent
const vec3 BRIGHT_GOLD = vec3(1.0, 0.9, 0.2);   // Brilliant bright gold

vec3 apply_gradient_tint(vec3 original, vec2 coord) {
    // Create dynamic radial gradient from center
    float center_distance = length(coord - vec2(0.5, 0.5));
    float gradient_strength = smoothstep(0.0, 0.9, center_distance);
    
    // Preserve luminance while adding intense color tint
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    vec3 tinted = mix(original, TINT_COLOR * (0.8 + luminance * 0.7), 0.22 * gradient_strength);
    
    // Add brilliant edge glow with bright accent
    float edge_glow = 1.0 - smoothstep(0.0, 0.35, center_distance);
    tinted += mix(ACCENT_COLOR, BRIGHT_GOLD, 0.6) * edge_glow * 0.12;
    
    // Add file manager-specific golden shimmer
    float shimmer = sin(coord.x * 20.0) * cos(coord.y * 16.0) * 0.5 + 0.5;
    float warm_glow = 1.0 - abs(coord.y - 0.5) * 1.5;
    tinted += TINT_COLOR * warm_glow * gradient_strength * 0.06;
    tinted += BRIGHT_GOLD * shimmer * edge_glow * 0.08;
    
    // Add diagonal sweep effect for luxury feel
    float diagonal = sin((coord.x + coord.y) * 12.0) * 0.5 + 0.5;
    tinted += TINT_COLOR * diagonal * gradient_strength * 0.05;
    
    // Add corner brilliance
    float corner_intensity = max(max(coord.x, 1.0 - coord.x), max(coord.y, 1.0 - coord.y));
    corner_intensity = smoothstep(0.75, 1.0, corner_intensity);
    tinted += BRIGHT_GOLD * corner_intensity * 0.08;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_gradient_tint(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
