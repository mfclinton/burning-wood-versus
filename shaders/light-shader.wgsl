const LIGHT_FALLOFF: f32 = 2.0;
const MAX_LIGHTS: u32 = 8;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    radius: f32,
}

struct Global {
    // Position of the camera in 3D space
    camera: vec3<f32>,
    // Size of the viewport in pixels (width, height)
    viewport: vec2<f32>,
    // Current tick or frame count
    tick: u32,
    num_lights: u32,
    ambient_light: vec3<f32>,
    lights: array<Light, 4>,
}

@group(0) @binding(0)
var<uniform> global: Global;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    // Position of the vertex in clip space
    @builtin(position) position: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.position = vec4<f32>(in.pos, 0.0, 1.0);
    out.uv = in.uv;

    return out;
}

@group(1) @binding(0)
var t_canvas: texture_2d<f32>;

@group(1) @binding(1)
var s_canvas: sampler;

fn calculate_light_contribution(light: Light, world_pos: vec2<f32>) -> vec3<f32> {
    let light_world_pos = light.position.xy;
    let distance = length(world_pos - light_world_pos);

    if distance > light.radius {
        return vec3<f32>(0.0);
    }
    
    let normalized_distance = distance / light.radius;
    let attenuation = 1.0 - pow(normalized_distance, LIGHT_FALLOFF);
    let final_intensity = attenuation * light.intensity;
    
    return light.color * final_intensity;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate World Position
    let screen_center = global.viewport.xy * 0.5;
    let screen_offset = in.uv * global.viewport.xy - screen_center;
    
    let world_pos = global.camera.xy + screen_offset / global.camera.z;

    // Calculate Light
    var final_light = global.ambient_light;
    for (var i: u32 = 0u; i < MAX_LIGHTS; i++) {
        if (i < global.num_lights) {
            final_light += calculate_light_contribution(global.lights[i], world_pos);
        }
    }
    final_light = clamp(final_light, vec3<f32>(0.0), vec3<f32>(1.0));
    
    // Adjust Color
    let base_color = textureSample(t_canvas, s_canvas, in.uv);
    let lit_color = base_color.rgb * final_light;
    return vec4<f32>(lit_color, base_color.a);
}
