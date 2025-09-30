use crate::{interval::Interval, vec3::Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Color(Vec3);

type RGB8 = (u8, u8, u8);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

impl Color {
    fn get_gamma_rgb8(&self) -> RGB8 {
        let r = linear_to_gamma(self.0.x());
        let g = linear_to_gamma(self.0.y());
        let b = linear_to_gamma(self.0.z());
        {
            #![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            const INTENSITY: Interval = Interval::new(0.0, 0.999);
            let red = (256.0 * INTENSITY.clamp(r)) as u8;
            let green = (256.0 * INTENSITY.clamp(g)) as u8;
            let blue = (256.0 * INTENSITY.clamp(b)) as u8;

            (red, green, blue)
        }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (red, green, blue) = self.get_gamma_rgb8();
        write!(f, "{red} {green} {blue}")
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.0 * rhs.0)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from(self.0 * rhs)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::from(self * rhs.0)
    }
}

impl std::ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
