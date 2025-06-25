#version 320 es
// Ambient Arcade Glow - "Nostalgic Nights"
// Warm, subtle pink ambient lighting inspired by classic arcade machines

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 WARM_PINK = vec3(1.0, 0.4, 0.6);     // Soft arcade pink
const vec3 MAGENTA_HUE = vec3(1.0, 0.2, 0.7);   // Gentle magenta accent

vec3 apply_ambient_arcade_glow(vec3 original, vec2 coord) {
    // Very gentle edge glow that mimics arcade cabinet lighting
    float edge_distance = min(min(coord.x, 1.0 - coord.x), min(coord.y, 1.0 - coord.y));
    float ambient_glow = 1.0 - smoothstep(0.0, 0.6, edge_distance);
    ambient_glow = pow(ambient_glow, 2.0) * 0.15; // Soft falloff
    
    // Preserve original luminance for natural look
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    
    // Very subtle warm pink tint across the entire surface
    vec3 tinted = mix(original, WARM_PINK * (0.7 + luminance * 0.3), 0.05);
    
    // Add gentle ambient glow around edges
    tinted += MAGENTA_HUE * ambient_glow;
    
    // Extremely subtle center highlight for depth
    float center_dist = distance(coord, vec2(0.5, 0.5));
    float center_glow = 1.0 - smoothstep(0.0, 0.8, center_dist);
    tinted += WARM_PINK * center_glow * 0.02;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_ambient_arcade_glow(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
