use crate::primatives::{ Point, Line, Circle, Rectangle };
use crate::image::{ Image, PixelPoint };

pub fn apply_point_to_image(point: &Point, weight: i32, color: &[u8; 3], image: &mut Image) {
    
    if weight == 0 {
        println!("Warning: Got a weight of 0 while trying to draw a point. ");
        return;
    }

    let real_weight: i32;
    if weight == 1 {
        real_weight = 1;
    } else {
        real_weight = weight / 2;
    }

    for y in -real_weight..real_weight {
        for x in -real_weight..real_weight {

            let mut pixel_point = PixelPoint::from_point(point, image);

            pixel_point.x += x;
            pixel_point.y += y;

            if pixel_point.x < 0 || pixel_point.y < 0 {
                continue;
            }

            let _ = image.set_pixel(pixel_point.x as usize, pixel_point.y as usize, &color);
        }
    }
}

pub fn apply_line_to_image(line: &Line, weight: i32, color: &[u8; 3], image: &mut Image) {

    let start = PixelPoint::from_point(&line.start, image);
    let end = PixelPoint::from_point(&line.end, image);

    draw_line(image, weight, start, end, color);
}

pub fn apply_rectangle_to_image(rect: &Rectangle, weight: i32, color: &[u8; 3], image: &mut Image) {
    for line in rect.to_lines() {
        apply_line_to_image(&line, weight, color, image);
    }
}

pub fn apply_circle_to_image(circle: &Circle, weight: i32, color: &[u8; 3], image: &mut Image) {
    for line in circle.to_lines(50) {
        apply_line_to_image(&line, weight, color, image);
    }
}

fn draw_line(image: &mut Image, scale: i32, start_point: PixelPoint, end_point: PixelPoint, color: &[u8; 3]) {
    if (end_point.y - start_point.y).abs() < (end_point.x - start_point.x).abs() {
        if start_point.x > end_point.x {
            draw_line_low(image, scale, end_point, start_point, color);
        } else {
            draw_line_low(image, scale, start_point, end_point, color);
        }
    } else {
        if start_point.y > end_point.y {
            draw_line_high(image, scale, end_point, start_point, color);
        } else {
            draw_line_high(image, scale, start_point, end_point, color);
        }
    }
}

fn draw_line_high(image: &mut Image, scale: i32, start_point: PixelPoint, end_point: PixelPoint, color: &[u8; 3]) {
    let mut delta_x = end_point.x - start_point.x;
    let delta_y = end_point.y - start_point.y;

    let mut x_itr = 1 as i32; 
    if delta_x < 0 {
        x_itr = -1;
        delta_x = -delta_x;
    }

    let mut distance = 2 * delta_x - delta_y;
    let mut x = start_point.x;

    for y in start_point.y..=end_point.y {

        draw_point(image, color, scale, PixelPoint { x: x, y: y });

        if distance > 0 {
            x = x + x_itr;
            distance = distance + (2 * (delta_x - delta_y));
        } else {
            distance = distance + 2 * delta_x;
        }
    }
}

fn draw_line_low(image: &mut Image, scale: i32, start_point: PixelPoint, end_point: PixelPoint, color: &[u8; 3]) {
    let delta_x = end_point.x - start_point.x;
    let mut delta_y = end_point.y - start_point.y;

    let mut y_itr = 1;
    if delta_y < 0 {
        y_itr = -1;
        delta_y = -delta_y;
    }

    let mut distance = (2 * delta_y) - delta_x;
    let mut y = start_point.y;
    for x in start_point.x..=end_point.x {
        
        draw_point(image, color, scale, PixelPoint { x: x, y: y });

        if distance > 0 {
            y = y + y_itr;
            distance = distance + (2 * (delta_y - delta_x))
        } else {
            distance = distance + 2 * delta_y;
        }
    } 
}

fn draw_point(image: &mut Image, color: &[u8; 3], size: i32, point: PixelPoint) {

    let radius = size / 4;
    for y_off in -radius..=radius {
        for x_off in -radius..=radius {
            let x = point.x + x_off;
            let y = point.y + y_off;

            if x < 0 || y < 0 {
                continue;
            }

            let _ = image.set_pixel(x as usize, y as usize, color);
        }
    }
}