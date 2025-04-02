use image::{RgbImage, Rgb};
use std::path::Path;
use std::time::Instant;
use num_complex::Complex64;
use std::fs;

fn mandelbrot(c: Complex64, max_iter: u32) -> u32 {
    let mut z = Complex64::new(0.0, 0.0);
    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn pixel_to_complex(x: u32, y: u32, width: u32, height: u32) -> Complex64 {
    let re = (x as f64 / width as f64) * 3.5 - 2.5;
    let im = (y as f64 / height as f64) * 2.0 - 1.0;
    Complex64::new(re, im)
}

fn main() {
    let width = 3840;
    let height = 2160;
    let max_iter = 3000;

    let now = Instant::now();
    let mut img = RgbImage::new(width, height);



    std::fs::create_dir_all("frames").unwrap();
    for y in 0..height {
        for x in 0..width {
            let c = pixel_to_complex(x, y, width, height);
            let iters = mandelbrot(c, max_iter);
            let pixel = if iters == max_iter {
                Rgb([0, 0, 0])
            } else {
                let cutoff = 512;
                let val = (255.0 * (iters.min(cutoff) as f64 / cutoff as f64)) as u8;
                Rgb([val, val, val])
            };
            img.put_pixel(x, y, pixel);
        }
    
        // 💾 Сохраняем прогресс каждые 100 строк
        if y % 100 == 0 {
            img.save(format!("frames/frame_{:04}.png", y)).unwrap();
        }
    }

    img.save(Path::new("mandelbrot_seq.png")).unwrap();
    println!("Последовательная версия завершена за: {:.2?}", now.elapsed());
}

