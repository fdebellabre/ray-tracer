use crate::hittable::{HitRecord, Hittable};
use crate::material::MaterialObject;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a MaterialObject,
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root1 = (-b - sqrtd) / a;
        let root2 = (-b + sqrtd) / a;

        let root = if t_min < root1 && root1 < t_max {
            root1
        } else if t_min < root2 && root2 < t_max {
            root2
        } else {
            return None;
        };

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let outside = r.direction().dot(outward_normal) < 0.;

        Some(HitRecord {
            t: root,
            p,
            normal: if outside {
                outward_normal
            } else {
                -outward_normal
            },
            outside,
            mat: self.material,
        })
    }
}
