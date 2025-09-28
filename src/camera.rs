use crate::{
    color::Color,
    hittable::{HitRecord, RayIntersection},
    interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use log::info;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Camera {
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
}

impl Camera {
    fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const CENTER: Point3 = Point3::new(0.0, 0.0, 0.0);

        let image_width_f64 = f64::from(image_width);

        // Image dimension calculations - calculate the height from aspect ratio and hardcoded width, and clamp image height to be at least 1
        let image_height_f64 = (image_width_f64 / aspect_ratio).max(1.0);

        #[allow(clippy::cast_possible_truncation)]
        let image_height = image_height_f64 as i32;

        // Camera calculations
        let viewport_width = VIEWPORT_HEIGHT * image_width_f64 / image_height_f64;
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
        let pixel_delta_u = viewport_u / image_width_f64;
        let pixel_delta_v = viewport_v / image_height_f64;
        let viewport_upper_left =
            CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            center: CENTER,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: f64::from(samples_per_pixel).recip(),
            image_width,
            image_height,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &impl RayIntersection) {
        // Output PPM image to standard output

        // PPM header
        // First line indicates colors are in ASCII
        // Second line indicates the width and height of the image
        // Third line indicates the maximum color value
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for y in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - y);

            for x in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += ray_color(&r, world);
                }

                println!("{}", self.pixel_samples_scale * pixel_color);
            }
        }

        info!("Image rendering complete");
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((f64::from(x) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(y) + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
}

fn ray_color(r: &Ray, world: &impl RayIntersection) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, interval::NON_NEGATIVE, &mut rec) {
        return 0.5 * (Color::from(rec.normal()) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

pub struct CameraBuilder(f64, i32, i32);

impl CameraBuilder {
    pub const fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        Self(aspect_ratio, image_width, samples_per_pixel)
    }

    pub const fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.0 = aspect_ratio;
        self
    }

    pub const fn image_width(mut self, image_width: i32) -> Self {
        self.1 = image_width;
        self
    }

    pub const fn samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.2 = samples_per_pixel;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(self.0, self.1, self.2)
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new(1.0, 100, 10)
    }
}
