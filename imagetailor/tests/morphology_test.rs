extern crate image;
extern crate imagetailor;

use std::fs::File;
use std::path::Path;
use image::{DynamicImage};

#[test]
fn dilate_test() {
    let src = "images/morph.png";
    let dst = "images/dilate.jpg";

    let mut image: DynamicImage = image::open(&Path::new(src)).unwrap();
    let ksize = 3;
    let kernel = vec![vec![1u8; ksize]; ksize];
    imagetailor::process::morphology::dilate(&mut image, kernel, 2);

    let ref mut fout = File::create(&Path::new(dst)).unwrap();
    let _ = image.save(fout, image::PNG);
    assert!(Path::new(dst).exists());
    assert!(true);
}
