#version 320 es
// Auroran Anima Shader - "The Crystal Mind"
// AI crystal personality: Living energy patterns, crystalline facets, neural networks

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
out vec4 fragColor;

const vec3 CRYSTAL_CYAN = vec3(0.0, 1.0, 0.9);     // Core crystal energy
const vec3 ANIMA_BLUE = vec3(0.2, 0.8, 1.0);       // AI consciousness
const vec3 POWER_WHITE = vec3(0.8, 1.0, 1.0);      // Energy discharge
const vec3 NEURAL_TEAL = vec3(0.0, 0.9, 0.7);      // Neural pathways

vec3 apply_anima_crystal_effect(vec3 original, vec2 coord) {
    // Simple radial gradient from center
    float center_distance = length(coord - vec2(0.5, 0.5));
    float crystal_core = 1.0 - smoothstep(0.0, 0.8, center_distance);
    
    // Mystical crystal facet patterns
    float facet_angle = atan(coord.y - 0.5, coord.x - 0.5);
    float crystal_facets = abs(sin(facet_angle * 6.0)) * 0.5 + 0.5;
    crystal_facets = smoothstep(0.3, 0.7, crystal_facets);
    
    // Refracted light patterns through crystal
    float light_refraction = sin(coord.x * 12.0 + facet_angle * 3.0) * cos(coord.y * 10.0 + facet_angle * 2.0);
    light_refraction = abs(light_refraction) * 0.5 + 0.5;
    light_refraction = smoothstep(0.4, 0.8, light_refraction);
    
    // Preserve original luminance
    float luminance = dot(original, vec3(0.299, 0.587, 0.114));
    
    // Subtle mystical crystal tint
    vec3 tinted = mix(original, CRYSTAL_CYAN * (0.8 + luminance * 0.3), 0.04 * crystal_core);
    
    // Add crystal facet reflections
    tinted += POWER_WHITE * crystal_facets * 0.03;
    
    // Add refracted light effects
    tinted += CRYSTAL_CYAN * light_refraction * 0.025;
    tinted += ANIMA_BLUE * light_refraction * crystal_facets * 0.02;
    
    // Subtle edge glow
    float edge_distance = min(min(coord.x, 1.0 - coord.x), min(coord.y, 1.0 - coord.y));
    float crystal_edge = 1.0 - smoothstep(0.0, 0.3, edge_distance);
    tinted += NEURAL_TEAL * crystal_edge * 0.015;
    
    return tinted;
}

void main() {
    vec4 pixColor = texture(tex, v_texcoord);
    vec3 color = apply_anima_crystal_effect(pixColor.rgb, v_texcoord);
    fragColor = vec4(color, pixColor.a);
}
