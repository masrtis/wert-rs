use crate::{
    color::Color,
    hittable::{HitRecord, RayIntersection},
    interval,
    material::Scatter,
    ray::Ray,
    scope_timer::ScopeTimer,
    vec3::{Point3, Vec3},
};
use log::info;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Basis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Basis {
    pub fn new(look_from: &Point3, look_at: &Point3, v_up: &Vec3) -> Self {
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Self { u, v, w }
    }

    pub const fn u(&self) -> &Vec3 {
        &self.u
    }

    pub const fn v(&self) -> &Vec3 {
        &self.v
    }

    pub const fn w(&self) -> &Vec3 {
        &self.w
    }
}

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
    max_depth: u32,
}

#[derive(Clone, Debug)]
struct NewCameraParameters {
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Vec3,
    pub aspect_ratio: f64,
    pub vertical_fov: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: u32,
}

impl Camera {
    fn new(params: &NewCameraParameters) -> Self {
        let _timer = ScopeTimer::new("Camera::new");

        let image_width_f64 = f64::from(params.image_width);

        // Image dimension calculations - calculate the height from aspect ratio and hardcoded width, and clamp image height to be at least 1
        let image_height_f64 = (image_width_f64 / params.aspect_ratio).max(1.0);

        #[allow(clippy::cast_possible_truncation)]
        let image_height = image_height_f64 as i32;

        // Camera calculations
        let focal_length = (params.look_from - params.look_at).length();
        let theta = params.vertical_fov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width_f64 / image_height_f64;
        let basis = Basis::new(&params.look_from, &params.look_at, &params.v_up);
        let viewport_u = viewport_width * basis.u();
        let viewport_v = -viewport_height * basis.v();
        let pixel_delta_u = viewport_u / image_width_f64;
        let pixel_delta_v = viewport_v / image_height_f64;
        let center = params.look_from;
        let viewport_upper_left =
            center - (focal_length * basis.w()) - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: f64::from(params.samples_per_pixel).recip(),
            image_width: params.image_width,
            image_height,
            samples_per_pixel: params.samples_per_pixel,
            max_depth: params.max_depth,
        }
    }

    pub fn render(&self, world: &impl RayIntersection) {
        let _timer = ScopeTimer::new("Camera::render");

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
                    pixel_color += ray_color(&r, self.max_depth, world);
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

        Ray::new(&ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
}

fn ray_color(r: &Ray, depth: u32, world: &impl RayIntersection) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();
    if world.hit(r, interval::ERROR_CORRECTED_NON_NEGATIVE, &mut rec) {
        let mut scattered = Ray::default();
        let mut attuentation = Color::default();

        if rec
            .material()
            .scatter(r, &rec, &mut attuentation, &mut scattered)
        {
            return attuentation * ray_color(&scattered, depth - 1, world);
        }

        return Color::default();
    }

    let unit_direction = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

pub struct CameraBuilder(NewCameraParameters);

impl CameraBuilder {
    fn new(params: &NewCameraParameters) -> Self {
        Self(params.clone())
    }

    pub const fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.0.aspect_ratio = aspect_ratio;
        self
    }

    pub const fn image_width(mut self, image_width: i32) -> Self {
        self.0.image_width = image_width;
        self
    }

    pub const fn samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.0.samples_per_pixel = samples_per_pixel;
        self
    }

    pub const fn max_depth(mut self, max_depth: u32) -> Self {
        self.0.max_depth = max_depth;
        self
    }

    pub const fn vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.0.vertical_fov = vertical_fov;
        self
    }

    pub const fn look_from(mut self, look_from: &Point3) -> Self {
        self.0.look_from = *look_from;
        self
    }

    pub const fn look_at(mut self, look_at: &Point3) -> Self {
        self.0.look_at = *look_at;
        self
    }

    pub const fn v_up(mut self, v_up: &Vec3) -> Self {
        self.0.v_up = *v_up;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(&self.0)
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new(&NewCameraParameters {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vertical_fov: 90.0,
            look_from: Point3::default(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
        })
    }
}
