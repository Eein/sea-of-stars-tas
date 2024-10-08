use puffin_egui::egui::emath::Numeric;
use quaternion_core::Vector3;

/// Vector 3 Magnitude
pub fn magnitude(vec3: &Vector3<f32>) -> f64 {
    let mag2 = vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2];
    mag2.to_f64().sqrt()
}
