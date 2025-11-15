use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use enum_dispatch::enum_dispatch;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat: Arc<Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }

    pub const fn p(&self) -> &Point3 {
        &self.p
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub const fn material(&self) -> &Arc<Material> {
        &self.mat
    }

    pub const fn front_face(&self) -> bool {
        self.front_face
    }
}

#[enum_dispatch(Hittable)]
pub trait RayIntersection {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<Material>,
}

impl Sphere {
    pub fn with_center_and_direction(
        center: &Point3,
        dir: &Vec3,
        radius: f64,
        mat: &Arc<Material>,
    ) -> Self {
        Self {
            center: Ray::new(center, dir),
            radius: radius.max(0.0),
            mat: mat.clone(),
        }
    }

    pub fn with_motion(
        center_1: &Point3,
        center_2: &Point3,
        radius: f64,
        mat: &Arc<Material>,
    ) -> Self {
        Self::with_center_and_direction(center_1, &(center_2 - center_1), radius, mat)
    }

    pub fn new(center: &Point3, radius: f64, mat: &Arc<Material>) -> Self {
        Self::with_center_and_direction(center, &Vec3::default(), radius, mat)
    }
}

impl RayIntersection for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.dir().length_squared();
        let h = oc.dot(r.dir());
        let c = self.radius.mul_add(-self.radius, oc.length_squared());
        let discriminant = h.mul_add(h, -(a * c));

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        let outward_normal = (hit_record.p - current_center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        hit_record.mat = self.mat.clone();

        true
    }
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Hittable {
    Sphere(Sphere),
}
