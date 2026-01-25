use crate::{
    aabb::AxisAlignedBoundingBox,
    hittable::{HitRecord, RayIntersection},
    interval::Interval,
    ray::Ray,
};
use rand::{RngExt, rng};
use std::{cmp::Ordering, sync::Arc};

fn box_compare(a: &dyn RayIntersection, b: &dyn RayIntersection, axis: usize) -> Ordering {
    let a_axis = a.bounding_box().axis_interval(axis);
    let b_axis = b.bounding_box().axis_interval(axis);
    a_axis.min.total_cmp(&b_axis.min)
}

#[derive(Debug)]
pub struct BvhNode {
    left: Arc<dyn RayIntersection>,
    right: Arc<dyn RayIntersection>,
    bbox: AxisAlignedBoundingBox,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn RayIntersection>]) -> Self {
        let (left, right): (Arc<dyn RayIntersection>, Arc<dyn RayIntersection>) = match objects {
            [] => panic!("Cannot create BVH node from empty slice"),
            [single] => (single.clone(), single.clone()),
            [first, second] => (first.clone(), second.clone()),
            _ => {
                let axis = rng().random_range(0..=2);
                objects.sort_by(|a, b| box_compare(a.as_ref(), b.as_ref(), axis));

                let mid = objects.len() / 2;
                let (left_slice, right_slice) = objects.split_at_mut(mid);
                (
                    Arc::new(Self::new(left_slice)),
                    Arc::new(Self::new(right_slice)),
                )
            }
        };

        let bbox = AxisAlignedBoundingBox::merge_boxes(&left.bounding_box(), &right.bounding_box());
        Self { left, right, bbox }
    }
}

impl From<&mut Vec<Arc<dyn RayIntersection>>> for BvhNode {
    fn from(value: &mut Vec<Arc<dyn RayIntersection>>) -> Self {
        Self::new(value.as_mut_slice())
    }
}

impl RayIntersection for BvhNode {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, hit_record);
        let right_interval_end = if hit_left { hit_record.t() } else { ray_t.max };
        let hit_right = self
            .right
            .hit(r, Interval::new(ray_t.min, right_interval_end), hit_record);

        hit_left || hit_right
    }
}
