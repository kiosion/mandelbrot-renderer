use wasm_bindgen::prelude::*;
use num::complex::Complex;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn compute(
    width: usize,
    height: usize,
    max_iter: usize,
    center_x: f64,
    center_y: f64,
    zoom: f64
) -> Vec<u8> {
    // If max_iter is > 2000, console error and return early, this will probably crash the browser
    if max_iter > 2000 {
        error("max_iter must be <= 2000");
    }

    let mut data = Vec::with_capacity(width * height * 4);

    let aspect_ratio = width as f64 / height as f64;

    let x_range = 3.5 / zoom; // width of x-values on the complex plane to include
    let y_range = x_range / aspect_ratio; // adjust height to maintain proper ratio

    let x_start = center_x - x_range / 2.0;
    let y_start = center_y - y_range / 2.0;

    for y in 0..height {
        for x in 0..width {
            let cx = x_start + (x as f64 / width as f64) * x_range;
            let cy = y_start + (y as f64 / height as f64) * y_range;

            let c = Complex::new(cx, cy);
            let mut z = Complex::new(0.0, 0.0);

            // opt for main cardioid
            let p = ((cx - 0.25).powi(2) + cy.powi(2)).sqrt();
            if cx <= p - 2.0 * p.powi(2) + 0.25 {
                data.extend_from_slice(&[0, 0, 0, 255]);
                continue;
            }

            // opt for period-2 bulb
            if (cx + 1.0).powi(2) + cy.powi(2) <= 0.0625 {
                data.extend_from_slice(&[0, 0, 0, 255]);
                continue;
            }

            let mut iter = 0;
            while z.norm_sqr() <= 4.0 && iter < max_iter {
                z = z * z + c;
                iter += 1;
            }

            let (r, g, b) = if iter == max_iter {
                (0, 0, 0) // interior if the set is black
            } else {
                // calc gradient w/ cosine waves
                let smoothed_iter = iter as f64 + 1.0 - (z.norm_sqr()).log2().log2() / 2.0;
                let red = (255.0 * (0.5 * (1.0 + (smoothed_iter * 0.01).cos()))).round() as u8;
                let green = (255.0 * (0.5 * (1.0 + (smoothed_iter * 0.02).cos()))).round() as u8;
                let blue = (255.0 * (0.5 * (1.0 + (smoothed_iter * 0.03).cos()))).round() as u8;
                (red, green, blue)
            };

            data.push(r);
            data.push(g);
            data.push(b);
            data.push(255); // alpha
        }
    }

    data
}
