@vertex
fn main(@location(0) in_position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in_position, 1.0);
}