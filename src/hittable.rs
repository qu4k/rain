use crate::{
  material::Material,
  ray::Ray,
  vec::{Point, Vec3},
};

#[derive(Copy, Clone)]
pub struct Hit<'a> {
  pub p: Point,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
  pub fn new(
    ray: &Ray,
    t: f64,
    p: Point,
    outward_normal: Vec3,
    material: &'a dyn Material,
  ) -> Self {
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
      material,
    }
  }
}

pub trait Hittable: Sync {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

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
