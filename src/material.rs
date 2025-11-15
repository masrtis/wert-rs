use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::random_unit_vector};
use enum_dispatch::enum_dispatch;
use rand::Rng;

#[enum_dispatch(Material)]
pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl From<Color> for Lambertian {
    fn from(albedo: Color) -> Self {
        Self::new(albedo)
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = *rec.normal();
        }

        *scattered = Ray::with_time(rec.p(), &scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Debug, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().reflect(rec.normal());
        let fuzzed = reflected.unit_vector() + (self.fuzz * random_unit_vector());
        *scattered = Ray::with_time(rec.p(), &fuzzed, r_in.time());
        *attenuation = self.albedo;
        scattered.dir().dot(rec.normal()) > 0.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl From<f64> for Dielectric {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_index_ratio = if rec.front_face() {
            self.refraction_index.recip()
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.dir().unit_vector();
        let cos_theta = (-unit_direction.dot(rec.normal())).min(1.0);
        let sin_theta = (cos_theta.mul_add(-cos_theta, 1.0)).sqrt();
        let can_refract = (refraction_index_ratio * sin_theta) <= 1.0;

        let mut rng = rand::rng();
        let refracted =
            if !can_refract || reflectance(cos_theta, refraction_index_ratio) > rng.random() {
                unit_direction.reflect(rec.normal())
            } else {
                unit_direction.refract(rec.normal(), refraction_index_ratio)
            };

        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::with_time(rec.p(), &refracted, r_in.time());
        true
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}
