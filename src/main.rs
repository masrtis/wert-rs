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

mod color;
mod hittable;
mod hittable_collection;
mod interval;
mod ray;
mod vec3;

use color::Color;
use hittable::{HitRecord, Hittable, RayIntersection, Sphere};
use hittable_collection::HittableCollection;
use log::info;
use ray::Ray;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray, world: &impl RayIntersection) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, interval::NON_NEGATIVE, &mut rec) {
        return 0.5 * (Color::from(rec.normal()) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Hardcoded constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_WIDTH_F64: f64 = 400.0;
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const CAMERA_CENTER: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Image dimension calculations - calculate the height from aspect ratio and hardcoded width, and clamp image height to be at least 1
    let image_height_f64 = (IMAGE_WIDTH_F64 / ASPECT_RATIO).max(1.0);

    #[allow(clippy::cast_possible_truncation)]
    let image_height = image_height_f64 as i32;

    // World setup
    let world = HittableCollection::from(
        [
            Hittable::from(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)),
            Hittable::from(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)),
        ]
        .as_slice(),
    );

    // Camera calculations
    let viewport_width = VIEWPORT_HEIGHT * IMAGE_WIDTH_F64 / image_height_f64;
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
    let pixel_delta_u = viewport_u / IMAGE_WIDTH_F64;
    let pixel_delta_v = viewport_v / image_height_f64;
    let viewport_upper_left =
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Initialize Logging
    colog::init();

    // Camera calculations

    // Output PPM image to standard output

    // PPM header
    // First line indicates colors are in ASCII
    // Second line indicates the width and height of the image
    // Third line indicates the maximum color value
    println!("P3");
    println!("{IMAGE_WIDTH} {image_height}");
    println!("255");

    for y in 0..image_height {
        info!("Scanlines remaining: {}", image_height - y);

        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (f64::from(x) * pixel_delta_u) + (f64::from(y) * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let r = Ray::new(CAMERA_CENTER, ray_direction);
            let pixel_color = ray_color(&r, &world);

            println!("{pixel_color}");
        }
    }

    info!("Image rendering complete");
}
