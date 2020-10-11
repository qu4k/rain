use crate::{material::Material, ray::Ray, vec::Point, vec::Vec3};

use super::{Hit, Primitive};

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub material: Box<dyn Material>,
}

impl Sphere {
  pub fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Self {
    Self {
      center,
      radius,
      material,
    }
  }
}

impl Primitive for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let oc: Vec3 = ray.orig - self.center;
    let a = ray.dir.len_squared();
    let half_b = oc.dot(&ray.dir);
    let c = oc.len_squared() - self.radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant > 0. {
      let root = discriminant.sqrt();
      let t = (-half_b - root) / a;
      if t < t_max && t > t_min {
        let p = ray.at(t);
        return Some(Hit::new(
          ray,
          t,
          p,
          (p - self.center) / self.radius,
          &*self.material,
        ));
      }
      let t = (-half_b + root) / a;
      if t < t_max && t > t_min {
        let p = ray.at(t);
        return Some(Hit::new(
          ray,
          t,
          p,
          (p - self.center) / self.radius,
          &*self.material,
        ));
      }
    }
    None
  }
}
