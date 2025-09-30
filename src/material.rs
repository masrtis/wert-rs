use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::random_unit_vector};

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attuentation: &mut Color,
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
        _r_in: &Ray,
        rec: &HitRecord,
        attuentation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        *scattered = Ray::new(rec.p(), scatter_direction);
        *attuentation = self.albedo;
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
        attuentation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().reflect(&rec.normal());
        let fuzzed = reflected.unit_vector() + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.p(), fuzzed);
        *attuentation = self.albedo;
        scattered.dir().dot(rec.normal()) > 0.0
    }
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}

impl Scatter for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attuentation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian(material) => material.scatter(r_in, rec, attuentation, scattered),
            Self::Metal(material) => material.scatter(r_in, rec, attuentation, scattered),
        }
    }
}
