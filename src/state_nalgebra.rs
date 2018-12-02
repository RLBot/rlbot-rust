use na::{Point3, Vector3};
use state;
use state_convert::{FromPoint3, FromVector3};

impl FromVector3<Vector3<f32>> for state::Vector3Partial {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: Some(v.x),
            y: Some(v.y),
            z: Some(v.z),
        }
    }
}

impl FromPoint3<Point3<f32>> for state::Vector3Partial {
    fn from(v: Point3<f32>) -> Self {
        Self {
            x: Some(v.x),
            y: Some(v.y),
            z: Some(v.z),
        }
    }
}
