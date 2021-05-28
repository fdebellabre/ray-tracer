use crate::{
    hittable::{HitRecord, Hittable},
    sphere::Sphere,
};
use crate::{ray::Ray, vec3::Vec3};

struct Rectangle(Vec3, Vec3, Vec3, Vec3);

impl Hittable for Rectangle {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        unimplemented!()
    }
}

pub enum HittableObject<'a> {
    Sphere(Sphere<'a>),
    Rectangle(Rectangle),
}

impl<'a> Hittable for HittableObject<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HittableObject::Sphere(sphere) => sphere.hit(r, t_min, t_max),
            HittableObject::Rectangle(rect) => rect.hit(r, t_min, t_max),
        }
    }
}

pub struct HittableList<T: Hittable> {
    pub list: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn add(&mut self, h: T) {
        self.list.push(h);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|object| object.hit(r, t_min, t_max))
            .min_by(|hit_record1, hit_record2| hit_record1.t.partial_cmp(&hit_record2.t).unwrap())
    }
}
