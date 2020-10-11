use rand::RngCore;

use crate::{hittable::Hit, ray::Ray, vec::Color, vec::Vec3};

use super::{Material, Scatter};

pub struct Lambertian {
  albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color) -> Box<Self> {
    Box::new(Self { albedo })
  }
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut dyn RngCore) -> Scatter {
    let scatter_dir = hit.normal + Vec3::random_unit(rng);
    Scatter {
      ray: Some(Ray::new(hit.p, scatter_dir)),
      color: self.albedo,
    }
  }
}
