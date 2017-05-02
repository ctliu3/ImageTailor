use std::collections::HashSet;

use image;
use image::{GenericImage, DynamicImage};

#[allow(unused_variables)]
pub fn dilate(image: &mut DynamicImage, kernel: Vec<Vec<u8>>, iterations: u16) {
    assert!(kernel.len() == kernel[0].len(), "kernel should have same width and height!");
    let (width, height) = image.dimensions();
    let ksize: u32 = kernel.len() as u32;

    for t in 0..iterations {
        let mut index_set = HashSet::new();
        for w in 0..(width - ksize) {
            for h in 0..(height - ksize) {
                let mut cnt = 0;
                for i in 0..ksize {
                    for j in 0..ksize {
                        let p = image.get_pixel(i + w, j + h);
                        if get_value(p) == 1 {
                            cnt += 1;
                        }
                    }
                }
                if cnt <= 0 {
                    continue;
                }
                for i in 0..ksize {
                    for j in 0..ksize {
                        index_set.insert((j + h) * width + i + w);
                    }
                }
            }
        }

        for index in index_set {
            let (h, w) = (index / width, index % width);
            let mut pixel = image.get_pixel(w, h);
            pixel.data[0] = 255u8;
            pixel.data[1] = 255u8;
            pixel.data[2] = 255u8;
            image.put_pixel(w, h, pixel);
        }
    }
}

#[allow(dead_code)]
fn inside(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

// 1: foreground
// 0: background
fn get_value(pixel: image::Rgba<u8>) -> i8 {
    if pixel.data[0] == 255 && pixel.data[1] == 255 && pixel.data[2] == 255 {
        return 1;
    } else if pixel.data[0] == 0 && pixel.data[1] == 0 && pixel.data[2] == 0 {
        return 0;
    }
    assert!(false, "unexpected pixel value");
    return 2;
}

