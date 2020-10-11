use rand::{Rng, RngCore};

use crate::{primitive::Hit, ray::Ray, vec::Color, vec::Vec3};

use super::{Material, Scatter};

pub struct Dielectric {
  ir: f64,
}

impl Dielectric {
  pub fn new(ir: f64) -> Box<Self> {
    Box::new(Self { ir })
  }
}

// Schlick's approximation for reflectance
fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
  let r0 = ((1. - refraction_ratio) / (1. + refraction_ratio)).powi(2);
  r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Material for Dielectric {
  fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut dyn RngCore) -> Scatter {
    let refraction_ratio = if hit.front_face {
      1. / self.ir
    } else {
      self.ir
    };

    let unit_direction = ray.dir.unit();
    let cos_th = ((-unit_direction).dot(&hit.normal)).min(1.);
    let sin_th = (1. - cos_th.powi(2)).sqrt();

    let cannot_refract = refraction_ratio * sin_th > 1.;

    let refracted = if cannot_refract || reflectance(cos_th, refraction_ratio) > rng.gen() {
      // cannot refract
      Vec3::reflect(&unit_direction, &hit.normal)
    } else {
      Vec3::refract(&unit_direction, &hit.normal, refraction_ratio)
    };

    Scatter {
      ray: Some(Ray::new(hit.p, refracted)),
      color: Color::new(1., 1., 1.),
    }
  }
}
