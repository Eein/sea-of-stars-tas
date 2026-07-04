//! Component accessors for [`vec3_rs::Vector3`].
//!
//! `vec3-rs` 0.4 dropped the `get_x`/`get_y`/`get_z` methods and made the
//! fields private, exposing components only through `From<Vector3>` tuple/array
//! conversions. This extension trait restores the ergonomic getters the rest of
//! the code relies on.

use vec3_rs::{Vector3, Vector3Coordinate};

pub trait Vector3Ext<T> {
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn get_z(&self) -> T;
}

impl<T: Vector3Coordinate> Vector3Ext<T> for Vector3<T> {
    fn get_x(&self) -> T {
        let (x, _, _): (T, T, T) = self.clone().into();
        x
    }

    fn get_y(&self) -> T {
        let (_, y, _): (T, T, T) = self.clone().into();
        y
    }

    fn get_z(&self) -> T {
        let (_, _, z): (T, T, T) = self.clone().into();
        z
    }
}
