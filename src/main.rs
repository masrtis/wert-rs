#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(clippy::exit)]

mod camera;
mod color;
mod hittable;
mod hittable_collection;
mod interval;
mod ray;
mod vec3;

fn main() {
    use camera::CameraBuilder;
    use hittable::{Hittable, Sphere};
    use hittable_collection::HittableCollection;
    use vec3::Point3;

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;

    // Initialize Logging
    colog::init();

    // World setup
    let world = HittableCollection::from(
        [
            Hittable::from(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)),
            Hittable::from(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)),
        ]
        .as_slice(),
    );

    let camera = CameraBuilder::default()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(IMAGE_WIDTH)
        .samples_per_pixel(100)
        .max_depth(50)
        .build();

    camera.render(&world);
}
