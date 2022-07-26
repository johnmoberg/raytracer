use std::fmt::Write;

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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(content, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    print!("{}", content);
    eprintln!("\nDone!");
}
