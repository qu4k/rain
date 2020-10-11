use rand::RngCore;

use crate::{primitive::Hit, ray::Ray, vec::Color, vec::Vec3};

use super::{Material, Scatter};

pub struct Metal {
  albedo: Color,
  fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Color, fuzz: f64) -> Box<Self> {
    let fuzz = fuzz.min(1.);
    Box::new(Self { albedo, fuzz })
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut dyn RngCore) -> Scatter {
    let reflected = Vec3::reflect(&ray.dir.unit(), &hit.normal);
    let scattered = Ray::new(
      hit.p,
      reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
    );
    Scatter {
      ray: if scattered.dir.dot(&hit.normal) > 0. {
        Some(scattered)
      } else {
        None
      },
      color: self.albedo,
    }
  }
}
