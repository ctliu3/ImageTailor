extern crate imagetailor;
extern crate image;

use std::fs::File;
use std::path::Path;
use image::{DynamicImage};

fn main() {
    let mut image: DynamicImage = image::open(&Path::new("examples/images/blur_src.jpg")).unwrap();
    imagetailor::process::filter::gaussian_blur(&mut image, 5, 1.0);

    let dest = "examples/images/blur_output.jpg";
    let ref mut fout = File::create(&Path::new(dest)).unwrap();
    let _ = image.save(fout, image::PNG);
    println!("done..\n");
}

