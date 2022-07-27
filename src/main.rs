extern crate nalgebra as na;
use na::Vector3;

trait Color {
    fn to_string(&self) -> String;
}

impl Color for Vector3<f32> {
    fn to_string(&self) -> String {
        let ir = (255.999 * self.x) as i32;
        let ig = (255.999 * self.y) as i32;
        let ib = (255.999 * self.z) as i32;

        format!("{} {} {}\n", ir, ig, ib)
    }
}

struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }

    fn color(&self) -> impl Color {
        let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, self);
        if t > 0.0 {
            let n = self.at(t) - Vector3::new(0.0, 0.0, -1.0);
            return 0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }

        let normalized_direction = self.direction.normalize();
        let t = 0.5 * (normalized_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.norm_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.norm_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1280;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Render
    let mut content = format!("P3\n {} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let color = ray.color();

            content.push_str(&Color::to_string(&color));
        }
    }

    print!("{}", content);
    eprintln!("\nDone!");
}
