use quaternion_core::{to_euler_angles, Quaternion, RotationSequence, RotationType, Vector3};
use std::f32::consts::PI;

/// Vector 3 Magnitude
pub fn magnitude(vec3: &Vector3<f32>) -> f32 {
    let mag2 = vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2];
    mag2.sqrt()
}

// Quaternion - Get Yaw Rotation
pub fn to_yaw(q: Quaternion<f32>) -> f32 {
    let angles = to_euler_angles(RotationType::Intrinsic, RotationSequence::XYZ, q);
    if angles[0] > 0.0 {
        return angles[1] - PI / 2.0;
    }
    PI - (angles[1] + PI / 2.0)
}
