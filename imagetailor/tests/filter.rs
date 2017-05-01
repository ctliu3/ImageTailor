extern crate image;
extern crate imagetailor;

use std::fs::File;
use std::path::Path;
use image::{DynamicImage};

#[test]
fn gaussian_blur_test() {
    let src = "examples/images/blur_src.jpg";
    let dst = "examples/images/blur_out.jpg";

    let mut image: DynamicImage = image::open(&Path::new(src)).unwrap();
    imagetailor::process::filter::gaussian_blur(&mut image, 5, 1.0);

    let ref mut fout = File::create(&Path::new(dst)).unwrap();
    let _ = image.save(fout, image::PNG);
    assert!(Path::new(dst).exists());
}
