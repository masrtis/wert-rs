use rand::{
    RngExt,
    distr::{
        Distribution, StandardUniform, Uniform,
        uniform::{
            Error as DistributionError, SampleBorrow, SampleUniform, UniformFloat, UniformSampler,
        },
    },
    rng,
};

#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub components: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: [x, y, z],
        }
    }

    pub const fn x(&self) -> f64 {
        self.components[0]
    }

    pub const fn y(&self) -> f64 {
        self.components[1]
    }

    pub const fn z(&self) -> f64 {
        self.components[2]
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x()
            .mul_add(rhs.x(), self.y().mul_add(rhs.y(), self.z() * rhs.z()))
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y().mul_add(rhs.z(), -self.z() * rhs.y()),
            self.z().mul_add(rhs.x(), -self.x() * rhs.z()),
            self.x().mul_add(rhs.y(), -self.y() * rhs.x()),
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub const fn near_zero(&self) -> bool {
        const S: f64 = 1.0e-8;
        self.components[0].abs() < S && self.components[1].abs() < S && self.components[2].abs() < S
    }

    pub fn reflect(&self, n: &Self) -> Self {
        *self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Self, refraction_index_ratio: f64) -> Self {
        let cos_theta = ((-self).dot(n)).min(1.0);
        let refract_perp = refraction_index_ratio * (self + cos_theta * n);
        let refract_parallel = -(1.0 - refract_perp.length_squared()).abs().sqrt() * n;
        refract_perp + refract_parallel
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Add<&Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self + *rhs
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl std::ops::AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.components[0] += rhs.x();
        self.components[1] += rhs.y();
        self.components[2] += rhs.z();
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Sub<&Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        self - *rhs
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl std::ops::SubAssign<&Self> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.components[0] -= rhs.components[0];
        self.components[1] -= rhs.components[1];
        self.components[2] -= rhs.components[2];
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl std::ops::Mul<&Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        self * *rhs
    }
}

impl std::ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        *self * rhs
    }
}

impl std::ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        *self * *rhs
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

impl std::ops::MulAssign<&Self> for Vec3 {
    fn mul_assign(&mut self, rhs: &Self) {
        self.components[0] *= rhs.components[0];
        self.components[1] *= rhs.components[1];
        self.components[2] *= rhs.components[2];
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Mul<&f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &f64) -> Self::Output {
        self * *rhs
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        *self * rhs
    }
}

impl std::ops::Mul<&f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f64) -> Self::Output {
        *self * *rhs
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.components[0] *= rhs;
        self.components[1] *= rhs;
        self.components[2] *= rhs;
    }
}

impl std::ops::MulAssign<&f64> for Vec3 {
    fn mul_assign(&mut self, rhs: &f64) {
        *self *= *rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    #[expect(
        clippy::suspicious_arithmetic_impl,
        reason = "Division is multiplication by the reciprocal"
    )]
    fn div(self, rhs: f64) -> Self::Output {
        rhs.recip() * self
    }
}

impl std::ops::Div<&f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: &f64) -> Self::Output {
        self / *rhs
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        *self / rhs
    }
}

impl std::ops::Div<&f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &f64) -> Self::Output {
        *self / *rhs
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    #[expect(
        clippy::suspicious_op_assign_impl,
        reason = "Division is multiplication by the reciprocal"
    )]
    fn div_assign(&mut self, rhs: f64) {
        *self *= rhs.recip();
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Distribution<Vec3> for StandardUniform {
    fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let distribution = Uniform::new(0.0, 1.0).unwrap();

        Vec3::new(
            distribution.sample(rng),
            distribution.sample(rng),
            distribution.sample(rng),
        )
    }
}

pub struct UniformVec3Sampler {
    x: UniformFloat<f64>,
    y: UniformFloat<f64>,
    z: UniformFloat<f64>,
}

impl UniformSampler for UniformVec3Sampler {
    type X = Vec3;

    fn new<B1: SampleBorrow<Self::X> + Sized, B2: SampleBorrow<Self::X> + Sized>(
        low: B1,
        high: B2,
    ) -> Result<Self, DistributionError> {
        let low = low.borrow();
        let high = high.borrow();

        Ok(Self {
            x: UniformFloat::new(low.x(), high.x())?,
            y: UniformFloat::new(low.y(), high.y())?,
            z: UniformFloat::new(low.z(), high.z())?,
        })
    }

    fn new_inclusive<B1: SampleBorrow<Self::X> + Sized, B2: SampleBorrow<Self::X> + Sized>(
        low: B1,
        high: B2,
    ) -> Result<Self, DistributionError> {
        let low = low.borrow();
        let high = high.borrow();

        Ok(Self {
            x: UniformFloat::new_inclusive(low.x(), high.x())?,
            y: UniformFloat::new_inclusive(low.y(), high.y())?,
            z: UniformFloat::new_inclusive(low.z(), high.z())?,
        })
    }

    fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec3::new(self.x.sample(rng), self.y.sample(rng), self.z.sample(rng))
    }
}

impl SampleUniform for Vec3 {
    type Sampler = UniformVec3Sampler;
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rng();
    loop {
        let p = rng.random_range(Vec3::new(-1.0, -1.0, -1.0)..Vec3::new(1.0, 1.0, 1.0));
        let length_sq = p.length_squared();
        if 1.0e-160 < length_sq && length_sq <= 1.0 {
            return p / length_sq.sqrt();
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rng();
    loop {
        let p = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
