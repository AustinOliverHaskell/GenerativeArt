use crate::primatives::Point;
use crate::pointcloud::PointCloud;

const GRAVITATIONAL_CONSTANT: f32 = 6.67e-11_f32;

struct ForceVector {
    direction: f32,
    magnitude: f32
}

fn calc_gravity_force_vector_from_gravity(
    point_a: &Point, point_a_mass: f32, 
    point_b: &Point, point_b_mass: f32) -> ForceVector {

    let magnitude: f32 = GRAVITATIONAL_CONSTANT * (point_a_mass * point_b_mass) / point_a.distance_between(point_b);

    let mut direction: f32 = 0.0;

    let slope = point_a.slope(point_b);
    if slope.is_none() {
        direction = 90.0;
    } else {
        slope = slope.unwrap();
    }
    

    ForceVector {
        magnitude: magnitude,
        direction: direction
    }

}

pub fn apply_gravity(point_cloud: &PointCloud) -> PointCloud {
    point_cloud.clone()
}