use na::Point3;
use na::Vector3;
use state;

impl From<Vector3<f32>> for state::Vector3Partial {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: Some(v.x),
            y: Some(v.y),
            z: Some(v.z),
        }
    }
}

impl From<Point3<f32>> for state::Vector3Partial {
    fn from(v: Point3<f32>) -> Self {
        Self {
            x: Some(v.x),
            y: Some(v.y),
            z: Some(v.z),
        }
    }
}
