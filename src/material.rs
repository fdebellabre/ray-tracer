use rand::Rng;

use crate::vec3::{Color, Vec3};
use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter<RNG: Rng>(&self, r_in: &Ray, rec: &HitRecord, rng: &mut RNG)
        -> Option<(Color, Ray)>;
}

pub enum MaterialObject {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for &MaterialObject {
    fn scatter<RNG: Rng>(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut RNG,
    ) -> Option<(Color, Ray)> {
        match self {
            MaterialObject::Lambertian(lambertian) => lambertian.scatter(r_in, rec, rng),
            MaterialObject::Metal(metal) => metal.scatter(r_in, rec, rng),
            MaterialObject::Dielectric(dielectric) => dielectric.scatter(r_in, rec, rng),
        }
    }
}

impl MaterialObject {
    pub fn lambertian(attenuation: Color) -> Self {
        MaterialObject::Lambertian(Lambertian { attenuation })
    }
    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        MaterialObject::Metal(Metal { albedo, fuzz })
    }
    pub fn dielectric(ir: f64, darken: f64) -> Self {
        MaterialObject::Dielectric(Dielectric { ir, darken })
    }
}

pub struct Lambertian {
    attenuation: Color,
}

impl Material for Lambertian {
    fn scatter<RNG: Rng>(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &mut RNG,
    ) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        let scattered = Ray(rec.p, scatter_direction);
        Some((self.attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter<RNG: Rng>(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut RNG,
    ) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).unit_vector();
        let scattered = Ray(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
        );

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64, // Index of Refraction
    darken: f64,
}

impl Material for Dielectric {
    fn scatter<RNG: Rng>(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut RNG,
    ) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.outside { 1.0 / self.ir } else { self.ir };
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // Use Schlick's approximation for reflectance.
        let r0 = (1. - refraction_ratio) / (1. + refraction_ratio);
        let r0 = r0 * r0;
        let r0 = r0 + (1. - r0) * (1. - cos_theta).powf(5.);

        let direction = if cannot_refract || r0 > rng.gen() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some((
            Color(self.darken, self.darken, self.darken),
            Ray(rec.p, direction),
        ))
    }
}

#[cfg(test)]
mod tests {

    use rand::{prelude::SmallRng, SeedableRng};

    use super::*;

    #[test]
    fn test_dielectric() {
        let ray = Ray(Vec3(-1., 0., 0.), Vec3(1.0, 0.0, 0.0));
        let hit_record = HitRecord {
            p: Vec3(0., 0., 0.),
            normal: Vec3(-1., -1., 0.).unit_vector(),
            mat: &MaterialObject::Dielectric(Dielectric {
                ir: 1.0,
                darken: 1.,
            }),
            t: 1.0,
            outside: false,
        };

        let mut rng = SmallRng::from_entropy();

        let (_color, ray) = hit_record.mat.scatter(&ray, &hit_record, &mut rng).unwrap();

        dbg!(ray.direction());

        assert!((ray.origin() - Vec3(0.0, 0.0, 0.0)).length_squared() < f64::EPSILON);
        assert!((ray.direction() - Vec3(1.0, 0.0, 0.0)).length_squared() < f64::EPSILON);
    }
}
