use crate::{
    aabb::AxisAlignedBoundingBox,
    hittable::{HitRecord, RayIntersection},
    interval::Interval,
    ray::Ray,
};

use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct HittableCollection {
    objects: Vec<Arc<dyn RayIntersection>>,
    bbox: AxisAlignedBoundingBox,
}

impl HittableCollection {
    pub fn add(&mut self, object: Arc<dyn RayIntersection>) {
        self.bbox = AxisAlignedBoundingBox::merge_boxes(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl RayIntersection for HittableCollection {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut curr_record = HitRecord::default();
        let mut found_hit = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
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

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bbox
    }
}
