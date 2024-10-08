use std::f32::consts::PI;
use vec3_rs::Vector3;
use libm::Libm;

#[derive(Default, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Quaternion {
    /// Convert Quaternion to Euler angles.
    pub fn to_angles(&self) -> Vector3<f32> {
        Vector3::new(
            Libm::<f32>::atan2(
                2.0 * (self.w * self.x + self.y * self.z),
                1.0 - 2.0 * (self.x * self.x + self.y * self.y),
            ),
            Libm::<f32>::asin(2.0 * (self.w * self.y - self.x * self.z)),
            Libm::<f32>::atan2(
                2.0 * (self.w * self.z + self.x * self.y),
                1.0 - 2.0 * (self.y * self.y + self.z * self.z),
            ),
        )
    }

    pub fn to_yaw(&self) -> f32 {
        let angles = self.to_angles();
        if angles.get_x() > 0.0 {
            return angles.get_y() - PI / 2.0;
        }
        PI - (angles.get_y() + PI / 2.0)
    }
}
