use std::collections::HashMap;

use image;
use image::{GenericImage, DynamicImage};

// OpenCV's methods of morphology can be seen here:
// http://docs.opencv.org/2.4/doc/tutorials/imgproc/erosion_dilatation/erosion_dilatation.html

pub fn dilate(image: &mut DynamicImage, kernel: Vec<Vec<u8>>, iterations: u16) {
    assert!(kernel.len() == kernel[0].len(), "kernel should have same width and height!");
    let (width, height) = image.dimensions();
    let ksize: u32 = kernel.len() as u32;

    for _ in 0..iterations {
        let mut index_pixel = HashMap::new();
        for w in 0..width {
            for h in 0..height {

                let mut p = image.get_pixel(w, h);
                for i in 0..ksize {
                    for j in 0..ksize {
                        let nw: i32 = (w + i) as i32 - ksize as i32 / 2;
                        let nh: i32 = (h + j) as i32 - ksize as i32 / 2;
                        if !inside(nw, nh, w as usize, h as usize) {
                            continue;
                        }
                        let neighbor = image.get_pixel(nw as u32, nh as u32);
                        if lessthen(neighbor, p) {
                            p = neighbor;
                        }
                    }
                }
                index_pixel.insert(h * width + w, p);
            }
        }

        for (index, pixel) in index_pixel {
            let (h, w) = (index / width, index % width);
            image.put_pixel(w, h, pixel);
        }
    }
}

pub fn erode(image: &mut DynamicImage, kernel: Vec<Vec<u8>>, iterations: u16) {
    assert!(kernel.len() == kernel[0].len(), "kernel should have same width and height!");
    let (width, height) = image.dimensions();
    let ksize: u32 = kernel.len() as u32;

    for _ in 0..iterations {
        let mut index_pixel = HashMap::new();
        for w in 0..width {
            for h in 0..height {

                let mut p = image.get_pixel(w, h);
                for i in 0..ksize {
                    for j in 0..ksize {
                        let nw: i32 = (w + i) as i32 - ksize as i32 / 2;
                        let nh: i32 = (h + j) as i32 - ksize as i32 / 2;
                        if !inside(nw, nh, w as usize, h as usize) {
                            continue;
                        }
                        let neighbor = image.get_pixel(nw as u32, nh as u32);
                        if lessthen(p, neighbor) {
                            p = neighbor;
                        }
                    }
                }
                index_pixel.insert(h * width + w, p);
            }
        }

        for (index, pixel) in index_pixel {
            let (h, w) = (index / width, index % width);
            image.put_pixel(w, h, pixel);
        }
    }
}

fn lessthen(pixela: image::Rgba<u8>, pixelb: image::Rgba<u8>) -> bool {
    let to_gray = |p: image::Rgba<u8>| {
        p.data[0] as usize * 229 + p.data[1] as usize * 587 + p.data[2] as usize * 114
    };
    let a: usize = to_gray(pixela);
    let b: usize = to_gray(pixelb);
    return a < b
}

fn inside(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}
