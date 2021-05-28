use hittable::Hittable;
use hittable_list::{HittableList, HittableObject};
use material::{Material, MaterialObject};
use minifb::{Key, WindowOptions};
use rand::{prelude::SmallRng, Rng, SeedableRng};
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

fn ray_color<T: Hittable, RNG: Rng>(
    r: &Ray,
    world: &HittableList<T>,
    depth: i32,
    rng: &mut RNG,
) -> Color {
    if depth <= 0 {
        return Color(0., 0., 0.);
    }

    if let Some(hit_rec) = world.hit(r, 0.0001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_rec.mat.scatter(r, &hit_rec, rng) {
            attenuation * ray_color(&scattered, world, depth - 1, rng)
        } else {
            Color(0., 0., 0.)
        }
    } else {
        // Background
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.1 + 1.);
        Color(1., 1., 1.) * (1. - t) + Color(0.5, 0.7, 1.) * t
    }
}

const IMG_WIDTH: usize = 600;

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let img_height = (IMG_WIDTH as f64 / aspect_ratio) as u32;
    let anti_alias_samples = 40;
    let max_depth = 10;

    // Materials
    let material_ground = MaterialObject::lambertian(Color(0.8, 0.8, 0.));
    let material_center = MaterialObject::lambertian(Color(0.1, 0.2, 0.5));
    let material_left = MaterialObject::dielectric(1.5, 0.97);
    let material_right = MaterialObject::metal(Color(0.8, 0.6, 0.2), 0.);

    // World
    let world = HittableList {
        list: vec![
            HittableObject::Sphere(Sphere {
                center: Point3(0., -100.5, -1.),
                radius: 100.,
                material: &material_ground,
            }),
            HittableObject::Sphere(Sphere {
                center: Point3(0., 0., -1.),
                radius: 0.5,
                material: &material_center,
            }),
            HittableObject::Sphere(Sphere {
                center: Point3(-1., 0., -1.),
                radius: 0.5,
                material: &material_left,
            }),
            HittableObject::Sphere(Sphere {
                center: Point3(-1., 0., -1.),
                radius: -0.45,
                material: &material_left,
            }),
            HittableObject::Sphere(Sphere {
                center: Point3(1., 0., -1.),
                radius: 0.5,
                material: &material_right,
            }),
        ],
    };

    let mut buffer = vec![0u32; IMG_WIDTH * img_height as usize];

    // Camera
    let cam = camera::Camera::new(
        Point3(-2., 2., 1.),
        Point3(0., 0., -1.),
        Vec3(0., 1., 0.),
        20.,
        aspect_ratio,
    );

    // Render
    let pixel_color_mapper = |x, y, rng: &mut SmallRng| {
        let mut pixel_color = Color(0., 0., 0.);
        for _s in 0..anti_alias_samples {
            let u = (x as f64 + rng.gen::<f64>() - 0.5) / IMG_WIDTH as f64;
            let v = (y as f64 + rng.gen::<f64>() - 0.5) / img_height as f64;
            let r = cam.get_ray(u, 1. - v);
            pixel_color += ray_color(&r, &world, max_depth, rng);
        }
        pixel_color / anti_alias_samples as f64
    };

    let seed: u64 = rand::thread_rng().gen();

    buffer
        .par_chunks_exact_mut(IMG_WIDTH)
        .enumerate()
        .for_each(|(y, line)| {
            let mut rng = SmallRng::seed_from_u64(seed.wrapping_add(y as u64));

            for (x, pixel) in line.iter_mut().enumerate() {
                let Vec3(r, g, b) = pixel_color_mapper(x, y, &mut rng);

                *pixel = (((r.sqrt() * 255.) as u32) << 16)
                    + (((g.sqrt() * 255.) as u32) << 8)
                    + ((b.sqrt() * 255.) as u32);
            }
        });

    let mut window = minifb::Window::new(
        "Ray tracer",
        IMG_WIDTH as usize,
        img_height as usize,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, IMG_WIDTH as usize, img_height as usize)
            .unwrap();
    }
}
