use crate::{
    aabb::AxisAlignedBoundingBox,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use enum_dispatch::enum_dispatch;
use std::{fmt::Debug, sync::Arc};

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
pub trait RayIntersection: Debug + Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AxisAlignedBoundingBox;
}

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<Material>,
    bbox: AxisAlignedBoundingBox,
}

impl Sphere {
    fn with_center_and_direction(
        center: &Point3,
        dir: &Vec3,
        radius: f64,
        mat: &Arc<Material>,
        bbox: AxisAlignedBoundingBox,
    ) -> Self {
        Self {
            center: Ray::new(center, dir),
            radius,
            mat: mat.clone(),
            bbox,
        }
    }

    pub fn with_motion(
        center_1: &Point3,
        center_2: &Point3,
        mut radius: f64,
        mat: &Arc<Material>,
    ) -> Self {
        radius = radius.max(0.0);
        let radius_vec = Vec3::new(radius, radius, radius);

        let box1 =
            AxisAlignedBoundingBox::from_points(&(center_1 - radius_vec), &(center_1 + radius_vec));
        let box2 =
            AxisAlignedBoundingBox::from_points(&(center_2 - radius_vec), &(center_2 + radius_vec));
        Self::with_center_and_direction(
            center_1,
            &(center_2 - center_1),
            radius,
            mat,
            AxisAlignedBoundingBox::merge_boxes(&box1, &box2),
        )
    }

    pub fn new(center: &Point3, mut radius: f64, mat: &Arc<Material>) -> Self {
        radius = radius.max(0.0);
        let radius_vec = Vec3::new(radius, radius, radius);

        Self::with_center_and_direction(
            center,
            &Vec3::default(),
            radius,
            mat,
            AxisAlignedBoundingBox::from_points(&(center - radius_vec), &(center + radius_vec)),
        )
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

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bbox
    }
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Hittable {
    Sphere(Sphere),
}
