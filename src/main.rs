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

mod window;

use clap::app_from_crate;
use image::{ImageBuffer, Rgba, RgbaImage};
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
  let args = app_from_crate!()
    .arg("--width [width] 'set width of output image (default 400)'")
    .arg("--height [height] 'set height of output image (default 255)'")
    .arg("--spp [spp] 'Samples per pixel (default 1000)'")
    .arg("--depth [depth] 'Max ray reflection depth (default 50)'")
    .arg("--window 'Show result in a window (default false)'")
    .arg("<output> 'Image output'")
    .get_matches();

  let ar = 16.0 / 9.0;

  let width = match args.value_of("width") {
    Some(arg) => arg.parse::<u32>().unwrap(),
    None => 400,
  };

  let height = match args.value_of("height") {
    Some(arg) => arg.parse::<u32>().unwrap(),
    None => (width as f64 / ar) as u32,
  };

  let ar = width as f64 / height as f64;

  let path = match args.value_of("output") {
    Some(arg) => arg,
    None => "out.png",
  };

  let spp = match args.value_of("spp") {
    Some(arg) => arg.parse::<u32>().unwrap(),
    None => 100,
  };

  let max_depth = match args.value_of("depth") {
    Some(arg) => arg.parse::<u32>().unwrap(),
    None => 50,
  };

  let window = args.is_present("window");

  // Image
  let mut img: RgbaImage = ImageBuffer::new(width, height);

  // Camera
  let camera = Camera::new(ar);

  let mut rng = rand::thread_rng();

  // World
  let mut world: Vec<Box<dyn Hittable>> = vec![];
  world.push(Box::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
  world.push(Box::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

  // Rendering
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
      *pixel = Rgba(color.rgb(spp))
    }
  }

  eprintln!();
  eprintln!("> Took {}ms", gen.elapsed().as_millis());
  eprintln!();
  eprintln!("Saving image...");

  // Save
  img.save(path).unwrap();

  eprintln!("Done. ({}ms)", gen.elapsed().as_millis());

  if window {
    eprintln!("Opening window...");
    // Render to screen
    window::display_image(&img);
  }
}
