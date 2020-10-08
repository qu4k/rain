use std::env;
use std::time::Instant;

use camera::Camera;
use hittable::{Hittable, Sphere};
use ray::Ray;
use vec::{Color, Point, Vec3};

mod ray;
mod vec;

mod camera;
mod hittable;

use image::{ImageBuffer, Rgb, RgbImage};

use rand::prelude::*;

fn ray_color(
  ray: &Ray,
  world: &impl Hittable,
  depth: u32,
  rng: &mut impl Rng,
) -> Color {
  if depth == 0 {
    return Color::new(0., 0., 0.);
  }
  match world.hit(ray, 0.001, f64::INFINITY) {
    Some(hit) => {
      let target = hit.p + hit.normal + Vec3::random_unit(rng);
      0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1, rng)
    }
    None => {
      let unit_dir = ray.dir.unit();
      let t = 0.5 * (unit_dir.y() + 1.);
      (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let desired_height = match args.get(1) {
    Some(arg) => arg.parse::<u32>().unwrap(),
    None => 400,
  };

  // Image
  let ar = 16.0 / 9.0;
  let width = desired_height;
  let height = (width as f64 / ar) as u32;
  let path = "image.png";
  let spp = 100;
  let max_depth = 50;

  let mut img: RgbImage = ImageBuffer::new(width, height);

  // Camera
  let camera = Camera::new(ar);

  let mut rng = rand::thread_rng();

  // World
  let mut world: Vec<Box<dyn Hittable>> = vec![];
  world.push(Box::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
  world.push(Box::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

  eprintln!("> Generating image ({}x{}) ...", width, height);
  let gen = Instant::now();

  for j in (0..height).rev() {
    eprint!("\r> Scanlines remaining: {} ", j);
    for i in 0..width {
      let mut color = Color::new(0., 0., 0.);
      for _ in 0..spp {
        let u = ((i as f64) + rng.gen::<f64>()) / (width - 1) as f64;
        let v = ((j as f64) + rng.gen::<f64>()) / (height - 1) as f64;
        let ray = camera.cast(u, v);
        color += ray_color(&ray, &world, max_depth, &mut rng);
      }

      let pixel = img.get_pixel_mut(i, height - 1 - j);
      *pixel = Rgb(color.rgb(spp))
    }
  }

  let gen_time = gen.elapsed().as_millis();

  eprintln!();
  eprintln!("> Took {}ms", gen_time);
  eprintln!();
  eprintln!("Saving image...");

  let saving = Instant::now();

  img.save(path).unwrap();

  let saving_time = saving.elapsed().as_millis();

  eprintln!(
    "Done. ({}ms gen + {}ms saving = {}ms)",
    gen_time,
    saving_time,
    gen_time + saving_time
  );
}
