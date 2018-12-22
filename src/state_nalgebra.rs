use crate::{
    na::{Point3, Vector3},
    state,
    state_convert::{FromPoint3, FromVector3},
};

impl FromVector3<Vector3<f32>> for state::Vector3Partial {
    fn from(v: Vector3<f32>) -> Self {
        Self::new().x(v.x).y(v.y).z(v.z)
    }
}

impl FromPoint3<Point3<f32>> for state::Vector3Partial {
    fn from(v: Point3<f32>) -> Self {
        Self::new().x(v.x).y(v.y).z(v.z)
    }
}
