use std::env;
use std::time::Instant;

use rain::camera::Camera;
use rain::hittable::{Hittable, Sphere};
use rain::material::Lambertian;
use rain::ray::Ray;
use rain::vec::{Color, Point};

use clap::app_from_crate;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32, rng: &mut impl Rng) -> Color {
  if depth == 0 {
    return Color::new(0., 0., 0.);
  }
  match world.hit(ray, 0.001, f64::INFINITY) {
    Some(hit) => {
      let scatter = hit.material.scatter(ray, &hit, rng);
      match scatter.ray {
        Some(ray) => scatter.color * ray_color(&ray, world, depth - 1, rng),
        None => Color::new(0., 0., 0.),
      }
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

  // Camera
  let camera = Camera::new(ar);

  // World
  let mut world: Vec<Box<dyn Hittable>> = vec![];
  world.push(Box::new(Sphere::new(
    Point::new(0., 0., -1.),
    0.5,
    Lambertian::new(Color::new(0.8, 0.8, 0.0)),
  )));
  world.push(Box::new(Sphere::new(
    Point::new(0., -100.5, -1.),
    100.,
    Lambertian::new(Color::new(0.0, 0.8, 0.8)),
  )));

  eprintln!("> Generating image ({}x{}) ...", width, height);

  let render = Instant::now();

  let pb = ProgressBar::new(height.into());
  pb.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
    .progress_chars("##-"));

  // Rendering
  let data: Vec<u8> = (0..height)
    .into_par_iter()
    .rev()
    .map(|j| {
      let row: Vec<u8> = (0..width)
        .into_par_iter()
        .map(|i| {
          let mut color = Color::new(0., 0., 0.);
          let mut rng = thread_rng();
          for _ in 0..spp {
            let u = ((i as f64) + rng.gen::<f64>()) / (width - 1) as f64;
            let v = ((j as f64) + rng.gen::<f64>()) / (height - 1) as f64;
            let ray = camera.cast(u, v);
            color += ray_color(&ray, &world, max_depth, &mut rng);
          }
          color.rgb(spp).to_vec()
        })
        .flatten()
        .collect();
      pb.inc(1);
      row
    })
    .flatten()
    .collect();
  
  pb.finish_and_clear();

  // Image
  let mut img: RgbImage = ImageBuffer::new(width, height);
  img.copy_from_slice(data.as_slice());

  eprintln!("> Took {}ms", render.elapsed().as_millis());
  eprintln!();
  eprintln!("Saving image...");

  // Save
  img.save(path).unwrap();

  eprintln!("Done. ({}ms)", render.elapsed().as_millis());
}
