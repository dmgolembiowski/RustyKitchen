extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;


#[derive(Copy, Clone)]
struct Point3D { x: f64, y: f64, z: f64 }

#[derive(Copy, Clone)]
struct Point2D { x: f64, y: f64 }

struct Frame { x_min: f64, x_max: f64,
               y_min: f64, y_max: f64 }

#[derive(Copy, Clone)]
struct Pixel { x_min: usize, y: usize }

fn project(point3d: Point3D) -> Point2D {
    Point2D { x: point3d.x / point3d.z, y: point3d.y / point3d.z }
}

fn normalize(point2d: Point2D, frame: Frame) -> Point2D {
    Point2D {
        x: 1.0 - (point2d.x - frame.x_min) / (frame.x_max - frame.x_min),
        y: (point2d.y - frame.y_min) / (frame.y_max - frame.y_min)
    }
}

fn rasterize_line(start: Pixel, end: Pixel, buffer: &mut [u8], size: usize) {
    let x1 = start.x as i32;
    let y1 = start.y as i32;
    let x2 = end.x as i32;

}


fn main() {

}

// https://github.com/alsor/durer_renderer/commit/94707881e3333e6473432b8379201334ee7514fe
