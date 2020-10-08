use crate::{
  ray::Ray,
  vec::{Point, Vec3},
};

#[derive(Copy, Clone)]
pub struct Hit {
  pub p: Point,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl Hit {
  pub fn new(ray: &Ray, t: f64, p: Point, outward_normal: Vec3) -> Self {
    let front_face = ray.dir.dot(&outward_normal) < 0.;
    let normal = if front_face {
      outward_normal
    } else {
      -outward_normal
    };
    Self {
      p,
      normal,
      t,
      front_face,
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Sphere {
  center: Point,
  radius: f64,
}

impl Sphere {
  pub fn new(center: Point, radius: f64) -> Self {
    Self { center, radius }
  }
}

impl Hittable for Sphere {
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
        return Some(Hit::new(
          ray,
          t,
          ray.at(t),
          (ray.at(t) - self.center) / self.radius,
        ));
      }
      let t = (-half_b + root) / a;
      if t < t_max && t > t_min {
        return Some(Hit::new(
          ray,
          t,
          ray.at(t),
          (ray.at(t) - self.center) / self.radius,
        ));
      }
    }
    None
  }
}

impl Hittable for Vec<Box<dyn Hittable>> {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut best = None;
    for child in self {
      if let Some(hit) = child.hit(ray, t_min, t_max) {
        match best {
          None => best = Some(hit),
          Some(prev) => {
            if hit.t < prev.t {
              best = Some(hit)
            }
          }
        }
      }
    }
    best
  }
}
