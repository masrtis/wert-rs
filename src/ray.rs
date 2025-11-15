use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub const fn with_time(origin: &Point3, dir: &Vec3, time: f64) -> Self {
        Self {
            origin: *origin,
            dir: *dir,
            time,
        }
    }

    pub const fn new(origin: &Point3, dir: &Vec3) -> Self {
        Self::with_time(origin, dir, 0.0)
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }

    pub const fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub const fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub const fn time(&self) -> f64 {
        self.time
    }
}
