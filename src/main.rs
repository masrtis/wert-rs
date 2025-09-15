#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![cfg_attr(
    not(test),
    warn(clippy::unwrap_used),
    warn(clippy::expect_used),
    warn(clippy::panic)
)]
#![warn(clippy::exit)]

use log::info;

mod color;
mod vec3;

fn main() {
    // Hardcoded width and height for now
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    colog::init();

    // Output PPM image to standard output

    // PPM header
    // First line indicates colors are in ASCII
    // Second line indicates the width and height of the image
    // Third line indicates the maximum color value
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let width_f64 = f64::from(IMAGE_WIDTH);
    let height_f64 = f64::from(IMAGE_HEIGHT);

    for y in 0..IMAGE_HEIGHT {
        info!("Scanlines remaining: {}", IMAGE_HEIGHT - y);

        for x in 0..IMAGE_WIDTH {
            let pixel_color = color::Color::new(
                f64::from(x) / (width_f64 - 1.0),
                f64::from(y) / (height_f64 - 1.0),
                0.0_f64,
            );

            println!("{pixel_color}");
        }
    }

    info!("Image rendering complete");
}
