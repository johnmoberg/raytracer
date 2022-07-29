extern crate nalgebra as na;
use std::thread::Thread;

use na::Vector3;
use rand::prelude::*;

trait Color {
    fn to_string(&self, samples_per_pixel: i32) -> String;
}

fn color_to_string(color: &Vector3<f32>, samples_per_pixel: i32) -> String {
    let (mut r, mut g, mut b) = (color.x, color.y, color.z);

    let scale = 1.0 / (samples_per_pixel as f32);
    r *= scale;
    g *= scale;
    b *= scale;

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    format!("{} {} {}\n", ir, ig, ib)
}

pub struct Camera {
    origin: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
}

impl Camera {
    fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f32> {
    loop {
        let mut v = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        v = 2.0 * v - Vector3::new(1.0, 1.0, 1.0);

        if v.norm_squared() < 1.0 {
            return v;
        }
    }
}

impl Ray {
    fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }

    fn color(&self, world: &dyn Hittable, depth: i32, rng: &mut ThreadRng) -> Vector3<f32> {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(self, 0.0, 1000.0) {
            let target = hit.p + hit.normal + random_in_unit_sphere(rng);
            return 0.5
                * Ray::color(
                    &Ray {
                        origin: hit.p,
                        direction: target - hit.p,
                    },
                    world,
                    depth - 1,
                    rng,
                );
        }

        let normalized_direction = self.direction.normalize();
        let t = 0.5 * (normalized_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

pub struct HitRecord {
    p: Vector3<f32>,
    normal: Vector3<f32>,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn from(ray: &Ray, p: Vector3<f32>, t: f32, outward_normal: Vector3<f32>) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        Self {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = -(half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);

        Some(HitRecord::from(ray, p, t, (p - self.center) / self.radius))
    }
}

pub struct Objects {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for Objects {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1024;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = Objects {
        objects: vec![
            Box::new(Sphere {
                center: Vector3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vector3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    // Camera
    let camera = Camera::new(ASPECT_RATIO);

    // Render
    let mut content = format!("P3\n {} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&world, MAX_DEPTH, &mut rng);
            }

            content.push_str(&color_to_string(&pixel_color, SAMPLES_PER_PIXEL));
        }
    }

    print!("{}", content);
    eprintln!("\nDone!");
}
