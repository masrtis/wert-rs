use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Clone, Copy, Debug, Default)]
pub struct AxisAlignedBoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AxisAlignedBoundingBox {
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        let x_axis = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let y_axis = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let z_axis = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        Self::new(x_axis, y_axis, z_axis)
    }

    pub const fn merge_boxes(that: &Self, other: &Self) -> Self {
        Self {
            x: Interval::merge(that.x, other.x),
            y: Interval::merge(that.y, other.y),
            z: Interval::merge(that.z, other.z),
        }
    }

    pub const fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let origin = r.origin();
        let dir = r.dir();

        for axis in 0..3 {
            let axis_interval = self.axis_interval(axis);
            let axis_dir_inverse = dir[axis].recip();

            let t0 = (axis_interval.min - origin[axis]) * axis_dir_inverse;
            let t1 = (axis_interval.max - origin[axis]) * axis_dir_inverse;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
