use crate::{
  material::Material,
  ray::Ray,
  vec::{Point, Vec3},
};

mod sphere;

pub use sphere::Sphere;

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

pub trait Primitive: Sync {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub type World = Vec<Box<dyn Primitive>>;

impl Primitive for Vec<Box<dyn Primitive>> {
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
