use rand::RngCore;

use crate::{
  ray::Ray,
  vec::{Point, Vec3},
};

pub struct Camera {
  pub origin: Point,
  pub lower_left_corner: Point,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub w: Vec3,
  pub u: Vec3,
  pub v: Vec3,
  pub lens_radius: f64,
}

impl Camera {
  pub fn new(
    look_from: Point,
    look_at: Point,
    vup: Vec3,
    fov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
  ) -> Self {
    let theta = fov.to_radians();
    let h = (theta / 2.).tan();

    let viewport_height = 2. * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).unit();
    let u = vup.cross(&w);
    let v = w.cross(&u);

    let origin = look_from;
    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * w;

    let lens_radius = aperture / 2.;

    Self {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
      w,
      u,
      v,
      lens_radius,
    }
  }
  pub fn cast(&self, s: f64, t: f64, rng: &mut dyn RngCore) -> Ray {
    let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk(rng);
    let offset: Vec3 = rd.x() * self.u + rd.y() * self.v;
    Ray::new(
      self.origin + offset,
      self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
    )
  }
}
