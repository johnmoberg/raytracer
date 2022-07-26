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

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut content = format!("P3\n {} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let g = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;
            let color = Vector3::new(r, g, b);

            content.push_str(&Color::to_string(&color));
        }
    }

    print!("{}", content);
    eprintln!("\nDone!");
}
