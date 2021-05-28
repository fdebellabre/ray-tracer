use rand::Rng;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{clone::Clone, ops::Range};

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn unit_vector(self) -> Self {
        self / self.length_squared().sqrt()
    }
    fn random<RNG: Rng>(rng: &mut RNG, range: Range<f64>) -> Self {
        Self(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }
    pub fn random_in_unit_sphere<RNG: Rng>(rng: &mut RNG) -> Self {
        loop {
            let rdom = Self::random(rng, -1.0..1.0);
            if rdom.length_squared() <= 1.0 {
                break rdom;
            }
        }
    }
    pub fn random_unit_vector<RNG: Rng>(rng: &mut RNG) -> Self {
        loop {
            let rdom = Self::random(rng, -1.0..1.0);
            if rdom.length_squared() < 1.0 {
                break rdom.unit_vector();
            }
        }
    }
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self += -other
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

pub use Vec3 as Point3; // 3D point
pub use Vec3 as Color; // RGB color

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dot() {
        let a = Vec3(1.0, 2.0, 0.0);
        let b = Vec3(1.0, 1.0, 0.0);

        assert!((a.dot(b) - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_refract() {
        let a = Vec3(1.0, 0.0, 0.0);
        let normal = Vec3(1.0, 1.0, 0.0).unit_vector();

        let b = a.refract(-normal, 1.0);

        dbg!(a, b);

        assert!((a - b).length_squared() < f64::EPSILON);
    }
}
