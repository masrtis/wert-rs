use crate::vec3::Vec3;

pub struct Color(Vec3);

pub type RGB8 = (u8, u8, u8);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

impl Color {
    fn get_rgb8(&self) -> RGB8 {
        let r = self.0.x();
        let g = self.0.y();
        let b = self.0.z();
        {
            #![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let red = (255.999 * r) as u8;
            let green = (255.999 * g) as u8;
            let blue = (255.999 * b) as u8;

            (red, green, blue)
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (red, green, blue) = self.get_rgb8();
        write!(f, "{red} {green} {blue}")
    }
}
