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
    use rand::{Rng, rng};
    use std::sync::Arc;
    use vec3::{Point3, Vec3};

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1200;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: u32 = 50;
    const VERTICAL_FOV: f64 = 20.0;
    const LOOK_FROM: Point3 = Point3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Point3 = Point3::new(0.0, 0.0, 0.0);
    const V_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const DEFOCUS_ANGLE: f64 = 0.6;
    const FOCUS_DIST: f64 = 10.0;

    // Initialize Logging
    colog::init();

    // World setup
    let mut hittables = Vec::new();

    let ground_material = Arc::new(Material::from(Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    hittables.push(Hittable::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    let mut rng = rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random();
            let center = Point3::new(
                f64::from(a) + rng.random_range(0.0..0.9),
                0.2,
                f64::from(b) + rng.random_range(0.0..0.9),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = match choose_mat {
                    ..0.8 => {
                        let albedo = Color::new(
                            rng.random::<f64>() * rng.random::<f64>(),
                            rng.random::<f64>() * rng.random::<f64>(),
                            rng.random::<f64>() * rng.random::<f64>(),
                        );
                        Arc::new(Material::from(Lambertian::new(albedo)))
                    }
                    0.8..0.95 => {
                        let albedo = Color::new(
                            rng.random_range(0.5..1.0),
                            rng.random_range(0.5..1.0),
                            rng.random_range(0.5..1.0),
                        );
                        let fuzz = rng.random::<f64>();
                        Arc::new(Material::from(Metal::new(albedo, fuzz)))
                    }
                    _ => Arc::new(Material::from(Dielectric::new(1.5))),
                };

                hittables.push(Hittable::from(Sphere::new(&center, 0.2, &sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Material::from(Dielectric::new(1.5)));
    let material2 = Arc::new(Material::from(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    let material3 = Arc::new(Material::from(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));

    hittables.push(Hittable::from(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        &material1,
    )));
    hittables.push(Hittable::from(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        &material2,
    )));
    hittables.push(Hittable::from(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        &material3,
    )));

    let world = HittableCollection::from(hittables.as_slice());

    let camera = CameraBuilder::default()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(IMAGE_WIDTH)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_depth(MAX_DEPTH)
        .vertical_fov(VERTICAL_FOV)
        .look_from(&LOOK_FROM)
        .look_at(&LOOK_AT)
        .v_up(&V_UP)
        .defocus_angle(DEFOCUS_ANGLE)
        .focus_distance(FOCUS_DIST)
        .build();

    camera.render(&world);
}
