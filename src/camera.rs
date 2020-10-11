use crate::{
  ray::Ray,
  vec::{Point, Vec3},
};

pub struct Camera {
  pub origin: Point,
  pub lower_left_corner: Point,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(ar: f64) -> Self {
    let viewport_height = 2.0;
    let viewport_width = ar * viewport_height;
    let focal_len = 1.0;

    let origin = Point::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_len);

    Self {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }
  pub fn cast(&self, u: f64, v: f64) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    )
  }
}
