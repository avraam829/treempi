use mpi::traits::*;
use image::{RgbImage, Rgb};
use num_complex::Complex64;
use std::path::Path;
use std::time::Instant;

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
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let rank = world.rank();
    let size = world.size();

    let width = 3840;
    let height = 2160;
    let max_iter = 3000;
    let rows_per_process = height / size as u32;
    let start_row = rank as u32 * rows_per_process;
    let end_row = if rank == size - 1 {
        height
    } else {
        start_row + rows_per_process
    };

    let now = Instant::now();
    let mut local_data: Vec<u8> = Vec::new();

    for y in start_row..end_row {
    for x in 0..width {
        let c = pixel_to_complex(x, y, width, height);
        let iters = mandelbrot(c, max_iter);
        let cutoff = 512;
        let val = (255.0 * (iters.min(cutoff) as f64 / cutoff as f64)) as u8;
        local_data.extend([val, val, val]);
    }

    // 💾 Сохраняем частичную картинку каждые 100 строк
    if (y - start_row) % 100 == 0 {
        let lines_done = (y - start_row + 1) as u32;
        let mut partial_img = RgbImage::new(width, height);
        for local_y in 0..lines_done {
            let global_y = start_row + local_y;
            for x in 0..width {
                let idx = (local_y * width + x) as usize * 3;
                let pixel = Rgb([
                    local_data[idx],
                    local_data[idx + 1],
                    local_data[idx + 2],
                ]);
                partial_img.put_pixel(x, local_y, pixel);
            }
        }
        partial_img
            .save(format!("frames/frame_rank{}_y{}.png", rank, y))
            .unwrap();
    }
}

    let root_rank = 0;
    let mut full_image: Option<Vec<u8>> = if rank == root_rank {
        Some(vec![0; (width * height * 3) as usize])
    } else {
        None
    };

    if rank == root_rank {
        world.process_at_rank(root_rank).gather_into_root(&local_data[..], full_image.as_mut().unwrap());
    } else {
        world.process_at_rank(root_rank).gather_into(&local_data[..]);
    }

    if rank == root_rank {
        let mut img = RgbImage::new(width, height);
        let data = full_image.unwrap();
        for (i, pixel) in img.pixels_mut().enumerate() {
            let base = i * 3;
            *pixel = Rgb([data[base], data[base + 1], data[base + 2]]);
        }
        img.save(Path::new("mandelbrot_mpi.png")).unwrap();
        println!("MPI-версия завершена за: {:.2?}", now.elapsed());
    }
}
