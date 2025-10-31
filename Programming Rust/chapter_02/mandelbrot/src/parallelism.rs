use num::Complex;
use crossbeam;
use crate::image::{pixel_to_point, render};

pub fn render_parallel(pixels: &mut [u8],
                       bounds: (usize, usize),
                       upper_left: Complex<f64>,
                       lower_right: Complex<f64>) {
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    
    let bands: Vec<&mut [u8]> = 
        pixels.chunks_mut(rows_per_band * bounds.0).collect();
    
    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            
            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    }).unwrap();
}