mod image;
mod image_fs;
mod colors;
mod primatives;
mod rasterizer;
mod pointcloud;
mod gravity_sim;

use image::Image;
use image_fs::write_image;

use primatives::*;
use pointcloud::PointCloud;

fn main() {
    println!("Making *Art*.... ");

    // 6ft x 6ft 
    let height: usize = 21600;
    let width: usize = 21600;

    //let height: usize = 10000;
    //let width: usize = 10000;
    let mut image = Image {
        image_data: vec![255; height * width * 3],
        width: width,
        height: height
    };

    //draw_vertical_line_of_squares(&mut image, 0.1);
    //draw_field_of_squares(&mut image, 10, 10);
    //draw_circle(&mut image, &colors::COLOR_BLACK);
    //draw_point_cloud(&mut image, &colors::COLOR_BLACK);
    draw_point_cloud_with_nearest_neighbors(&mut image, &colors::COLOR_BLACK, 0.05, 4);

    write_image("img.png", image);
}

fn draw_point_cloud_with_nearest_neighbors(image: &mut Image, color: &[u8; 3], max_distance: f32, stroke: i32) {
    let cloud = PointCloud::new(2000);

    let mut line_list: Vec<Line> = Vec::new();
    for point in &cloud.point_list {
        for other_point in &cloud.point_list {
            if point.distance_between(other_point) < max_distance {
                line_list.push(Line {
                    start: point.clone(),
                    end: other_point.clone()
                })
            }
        }
    }

    for line in line_list {
        rasterizer::apply_line_to_image(&line, stroke, color, image);
    }
}

fn draw_point_cloud(image: &mut Image, color: &[u8; 3]) {
    let cloud = PointCloud::new(100);

    for point in cloud.point_list {
        println!("Drawing point {:?}", &point);
        rasterizer::apply_point_to_image(&point, 4, color, image);
    }
}

fn draw_circle(image: &mut Image, color: &[u8; 3]) {
    let circle = Circle {
        center: Point {
            x: 0.5, 
            y: 0.5
        },
        radius: 0.5
    };

    rasterizer::apply_circle_to_image(&circle, 3, color, image);
}

fn draw_vertical_line_of_squares(image: &mut Image, square_size: f32) {
    
    let mut y: f32 = 0.0;
    let mut rotation: f32 = 0.0;
    while y <= 0.9 {
        let rect = Rectangle {
            center: Point {
                x: 0.5,
                y: y
            },
            width: square_size,
            height: square_size, 
            rotation: rotation
        };

        rasterizer::apply_rectangle_to_image(&rect, 2, &colors::COLOR_BLACK, image);

        rotation += 1.0;

        if y < 0.2 {
            y += 0.001;
        } else if y < 0.6 {
            y += 0.005;
        } else {
            y += 0.01;
        }
    }

}

#[allow(unused)]
fn draw_field_of_squares(image: &mut Image, square_size: usize, padding: usize) {
    
    let width: f32 = 1.0 / (image.width as f32  / square_size as f32);
    let height: f32 = 1.0 / (image.height as f32 / square_size as f32);

    let x_padding: f32 = 1.0 / (image.width as f32 / padding as f32);
    let y_padding: f32 = 1.0 / (image.height as f32 / padding as f32);
    
    let mut rotation = 0.0;
    let mut y: f32 = 0.0;
    while y <= 1.0 {
        let mut x: f32 = 0.0;
        while x <= 1.0 {
            let rect = Rectangle {
                center: Point {
                    x: x,
                    y: y
                },
                width: width,
                height: height, 
                rotation: rotation
            };

            rasterizer::apply_rectangle_to_image(&rect, 2, &colors::COLOR_BLACK, image);

            x += width + x_padding;
            rotation += 9.0;
        }
        y += height + y_padding;
    }
}

