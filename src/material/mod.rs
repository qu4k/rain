use rand::RngCore;

use crate::{hittable::Hit, ray::Ray, vec::Vec3};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
  pub color: Vec3,
  pub ray: Option<Ray>,
}

pub trait Material: Sync {
  fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut dyn RngCore) -> Scatter;
}
