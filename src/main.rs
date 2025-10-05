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
mod material;
mod ray;
mod scope_timer;
mod vec3;

fn main() {
    use camera::CameraBuilder;
    use color::Color;
    use hittable::{Hittable, Sphere};
    use hittable_collection::HittableCollection;
    use material::{Dielectric, Lambertian, Material, Metal};
    use std::sync::Arc;
    use vec3::{Point3, Vec3};

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;

    // Initialize Logging
    colog::init();

    // World setup
    let material_ground = Arc::new(Material::from(Lambertian::from(Color::new(0.8, 0.8, 0.0))));
    let material_center = Arc::new(Material::from(Lambertian::from(Color::new(0.1, 0.2, 0.5))));
    let material_left = Arc::new(Material::from(Dielectric::from(1.50)));
    let material_bubble = Arc::new(Material::from(Dielectric::from(1.50_f64.recip())));
    let material_right = Arc::new(Material::from(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)));

    let world = HittableCollection::from(
        [
            Hittable::from(Sphere::new(
                &Point3::new(0.0, -100.5, -1.0),
                100.0,
                &material_ground,
            )),
            Hittable::from(Sphere::new(
                &Point3::new(0.0, 0.0, -1.2),
                0.5,
                &material_center,
            )),
            Hittable::from(Sphere::new(
                &Point3::new(-1.0, 0.0, -1.0),
                0.5,
                &material_left,
            )),
            Hittable::from(Sphere::new(
                &Point3::new(-1.0, 0.0, -1.0),
                0.4,
                &material_bubble,
            )),
            Hittable::from(Sphere::new(
                &Point3::new(1.0, 0.0, -1.0),
                0.5,
                &material_right,
            )),
        ]
        .as_slice(),
    );

    let camera = CameraBuilder::default()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(IMAGE_WIDTH)
        .samples_per_pixel(100)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(&Point3::new(-2.0, 2.0, 1.0))
        .look_at(&Point3::new(0.0, 0.0, -1.0))
        .v_up(&Vec3::new(0.0, 1.0, 0.0))
        .build();

    camera.render(&world);
}
