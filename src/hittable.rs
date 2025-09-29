use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Debug, Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub const fn p(&self) -> Point3 {
        self.p
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait RayIntersection {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: *center,
            radius: radius.max(0.0),
        }
    }
}

impl RayIntersection for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.dir().length_squared();
        let h = oc.dot(*r.dir());
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
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);

        true
    }
}

#[derive(Clone, Debug)]
pub enum Hittable {
    Sphere(Sphere),
}

impl RayIntersection for Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        match self {
            Self::Sphere(s) => s.hit(r, ray_t, hit_record),
        }
    }
}

impl From<Sphere> for Hittable {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
    }
}
