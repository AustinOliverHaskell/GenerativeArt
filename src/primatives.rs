#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32, 
    pub y: f32
}

#[derive(Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point
}

#[derive(Clone)]
pub struct Rectangle {
    pub center: Point,
    pub height: f32, 
    pub width: f32,
    pub rotation: f32
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * 0.0174533
}

impl Point {
    pub fn rotate_around(self: &Self, anchor: &Self, angle: f32) -> Self {
        let mut point = self.clone();

        let angle_in_radians = degrees_to_radians(angle);

        let sin = angle_in_radians.sin();
        let cos = angle_in_radians.cos();

        point.x = point.x - anchor.x;
        point.y = point.y - anchor.y;

        let x = point.x * cos - point.y * sin;
        let y = point.x * sin + point.y * cos;

        point.x = x + anchor.x;
        point.y = y + anchor.y;

        point
    }

    pub fn random() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        Point {
            x: rng.gen(),
            y: rng.gen()
        }
    }

    pub fn distance_between(self: &Self, other: &Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn slope(self: &Self, other: &Self) -> Option<f32> {

        // @warning: Float compare - Austin Haskell 
        if other.x - self.x == 0.0 {
            // Undefined
            return None;
        }
        Some((other.y - self.y) / (other.x - self.x))
    }
}

impl Rectangle {

    pub fn to_lines(self: &Self) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();

        let mut a = Point {
            x: self.center.x - self.width / 2.0,
            y: self.center.y - self.height / 2.0
        };

        let mut b = Point {
            x: self.center.x - self.width / 2.0,
            y: self.center.y + self.height / 2.0
        };

        let mut c = Point {
            x: self.center.x + self.width / 2.0,
            y: self.center.y + self.height / 2.0
        };

        let mut d = Point {
            x: self.center.x + self.width / 2.0,
            y: self.center.y - self.height / 2.0
        };

        a = a.rotate_around(&self.center, self.rotation);
        b = b.rotate_around(&self.center, self.rotation);
        c = c.rotate_around(&self.center, self.rotation);
        d = d.rotate_around(&self.center, self.rotation);

        lines.push(Line {
            start: a.clone(),
            end: b.clone()
        });
        lines.push(Line {
            start: b.clone(),
            end: c.clone()
        });
        lines.push(Line {
            start: c.clone(),
            end: d.clone()
        });
        lines.push(Line {
            start: d.clone(),
            end: a.clone()
        });

        lines
    }

    pub fn rotate(self: &Self, angle: f32) -> Self {
        let mut copy = self.clone();
        copy.rotation = angle;

        copy
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f32
}

impl Circle {
    pub fn to_lines(self: &Self, point_count: u16) -> Vec<Line> {

        let mut lines: Vec<Line> = Vec::new();

        let rotation_steps: f32 = 360.0 / point_count as f32;

        let mut prev_point: Option<Point> = None;
        for n in 0..point_count {
            let rotation = rotation_steps * n as f32;

            let mut point = Point {
                x: self.center.x + self.radius,
                y: self.center.y
            };

            point = point.rotate_around(&self.center, rotation);

            if prev_point.is_none() {
                prev_point = Some(point);
                continue;
            }

            lines.push(Line {
                start: prev_point.clone().unwrap(), 
                end: point.clone()
            });

            prev_point = Some(point);
        }

        lines.push(Line {
            start: prev_point.unwrap(),
            end: Point {
                x: self.center.x + self.radius,
                y: self.center.y
            }
        });

        lines
    }
}

