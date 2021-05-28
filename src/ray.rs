use crate::vec3::Vec3;
pub struct Ray(pub Vec3, pub Vec3);

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.0 + self.1 * t
    }
    pub fn origin(&self) -> Vec3 {
        self.0
    }
    pub fn direction(&self) -> Vec3 {
        self.1
    }
}
