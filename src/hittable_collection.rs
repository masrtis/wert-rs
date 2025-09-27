use crate::{
    hittable::{HitRecord, Hittable, RayIntersection},
    interval::Interval,
    ray::Ray,
};

#[derive(Clone, Debug, Default)]
pub struct HittableCollection(Vec<Hittable>);

impl From<&[Hittable]> for HittableCollection {
    fn from(hittables: &[Hittable]) -> Self {
        Self(hittables.to_vec())
    }
}

impl From<&Hittable> for HittableCollection {
    fn from(h: &Hittable) -> Self {
        Self(vec![h.clone()])
    }
}

impl RayIntersection for HittableCollection {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut curr_record = HitRecord::default();
        let mut found_hit = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.0 {
            if object.hit(
                r,
                Interval::new(ray_t.min, closest_so_far),
                &mut curr_record,
            ) {
                found_hit = true;
                closest_so_far = curr_record.t();
                *rec = curr_record.clone();
            }
        }

        found_hit
    }
}
