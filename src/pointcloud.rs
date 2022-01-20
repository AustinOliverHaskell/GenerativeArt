use crate::primatives::Point;

#[derive(Clone)]
pub struct PointCloud {
    pub point_list: Vec<Point>
}

impl PointCloud {
    pub fn new(number: u32) -> Self {
        let mut point_list: Vec<Point> = Vec::new();

        for _ in 0..number {
            point_list.push(Point::random());
        }

        PointCloud {
            point_list: point_list
        }
    }
}