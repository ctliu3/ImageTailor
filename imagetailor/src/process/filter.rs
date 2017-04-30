
use image::{GenericImage, DynamicImage};

pub fn gaussian_blur(image: &mut DynamicImage, ksize: usize, sigma: f32) {
    let (width, height) = image.dimensions();

    let kernel = get_gaussian_kerenl(ksize, sigma);

    let inside = |w: i32, h: i32| {
        w >= 0 && w < width as i32 && h >= 0 && h < height as i32
    };
    let clamp = |x: f32| {
        if x < 0f32 {
            return 0u8;
        } else if x > 255f32 {
            return 255u8;
        }
        return x as u8;
    };

    for w in 0..width {
        for h in 0..height {
            let mut pixel = image.get_pixel(w, h);
            let mut r = 0f32;
            let mut g = 0f32;
            let mut b = 0f32;

            for i in 0..ksize {
                for j in 0..ksize {
                    let x = w as i32 - (ksize / 2) as i32 + i as i32;
                    let y = h as i32 - (ksize / 2) as i32 + j as i32;
                    if inside(x, y) {
                        let offset_pixel = image.get_pixel(x as u32, y as u32);
                        r += kernel[i * ksize + j] * offset_pixel.data[0] as f32;
                        g += kernel[i * ksize + j] * offset_pixel.data[1] as f32;
                        b += kernel[i * ksize + j] * offset_pixel.data[2] as f32;
                    }
                }
            }

            pixel.data[0] = clamp(r);
            pixel.data[1] = clamp(g);
            pixel.data[2] = clamp(b);
            image.put_pixel(w, h, pixel);
        }
    }
}

fn get_gaussian_kerenl(ksize: usize, sigma: f32) -> Vec<f32> {
    if ksize == 0 {
        panic!("kernel size should be larger than 0, given {:?}", ksize);
    }
    if ksize & 1 != 1 {
        panic!("kernel size should be an odd value, given {:?}", ksize);
    }

    let mut kernel = vec![0f32; ksize * ksize];
    let mut norm_factor = 0f32;
    let cx: usize = ksize / 2;
    let cy = cx;

    for i in 0..ksize {
        for j in 0..ksize {
            let t1 = (cx as i32 - i as i32).pow(2) + (cy as i32 - j as i32).pow(2);
            let v: f32 = (-t1 as f32 / (2.0 * sigma * sigma)).exp();
            kernel[i * ksize + j] = v;
            norm_factor += v;
        }
    }
    for i in 0..ksize {
        for j in 0..ksize {
            kernel[i * ksize + j] /= norm_factor;
        }
    }
    return kernel;
}

