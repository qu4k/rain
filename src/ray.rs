use crate::vec::{Point, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
  pub orig: Point,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(orig: Point, dir: Vec3) -> Self {
    Self { orig, dir }
  }

  pub fn at(&self, t: f64) -> Point {
    self.orig + t * self.dir
  }
}
