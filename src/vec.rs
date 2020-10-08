use std::fmt;
use std::ops;

use rand::Rng;

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self(x, y, z)
  }

  pub fn random(rng: &mut impl Rng) -> Self {
    Self(rng.gen(), rng.gen(), rng.gen())
  }

  pub fn random_range(rng: &mut impl Rng, min: f64, max: f64) -> Self {
    Self(
      rng.gen_range(min, max),
      rng.gen_range(min, max),
      rng.gen_range(min, max),
    )
  }

  pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
    loop {
      let p = 2.0 * Self::random(rng) - Self(1.0, 1.0, 1.0);
      if p.len_squared() < 1.0 {
        return p;
      }
    }
  }

  pub fn random_unit(rng: &mut impl Rng) -> Self {
    let a: f64 = rng.gen_range(0., 2. * std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1., 1.);
    let r = (1. - z.powi(2)).sqrt();
    Self(r * a.cos(), r * a.sin(), z)
  }

  pub fn x(&self) -> f64 {
    self.0
  }
  pub fn y(&self) -> f64 {
    self.1
  }
  pub fn z(&self) -> f64 {
    self.2
  }

  pub fn len(&self) -> f64 {
    self.len_squared().sqrt()
  }

  pub fn len_squared(&self) -> f64 {
    self.dot(self)
  }

  pub fn dot(&self, rhs: &Self) -> f64 {
    self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
  }

  pub fn cross(&self, rhs: &Self) -> Self {
    Self(
      self.1 * rhs.2 - self.2 * rhs.1,
      self.2 * rhs.0 - self.0 * rhs.2,
      self.0 * rhs.1 - self.1 - rhs.0,
    )
  }

  pub fn unit(&self) -> Self {
    *self / self.len()
  }
}

impl ops::Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self(-self.0, -self.1, -self.2)
  }
}

// Add Vec3

impl ops::Add<Vec3> for Vec3 {
  type Output = Self;

  fn add(self, rhs: Vec3) -> Self::Output {
    Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

impl ops::AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
    self.1 += rhs.1;
    self.2 += rhs.2;
  }
}

// Sub Vec3

impl ops::Sub<Vec3> for Vec3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}

impl ops::SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 -= rhs.0;
    self.1 -= rhs.1;
    self.2 -= rhs.2;
  }
}

// Mul Vec3

impl ops::Mul<Vec3> for Vec3 {
  type Output = Self;

  fn mul(self, rhs: Vec3) -> Self::Output {
    Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
  }
}

impl ops::MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, rhs: Vec3) {
    self.0 *= rhs.0;
    self.1 *= rhs.1;
    self.2 *= rhs.2;
  }
}

// Div Vec3

impl ops::Div<Vec3> for Vec3 {
  type Output = Self;

  fn div(self, rhs: Vec3) -> Self::Output {
    Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
  }
}

impl ops::DivAssign<Vec3> for Vec3 {
  fn div_assign(&mut self, rhs: Vec3) {
    self.0 /= rhs.0;
    self.1 /= rhs.1;
    self.2 /= rhs.2;
  }
}

// Mul f64

impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
  }
}

// Div f64

impl ops::Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
  }
}

impl ops::DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, rhs: f64) {
    self.0 /= rhs;
    self.1 /= rhs;
    self.2 /= rhs;
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.0)
      .field("y", &self.1)
      .field("z", &self.2)
      .finish()
  }
}

impl Default for Vec3 {
  fn default() -> Self {
    Self(0., 0., 0.)
  }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
  if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}

impl Color {
  pub fn rgb(&self, spp: u32) -> [u8; 4] {
    let scale = 1. / spp as f64;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let r = (self.0 * scale).sqrt();
    let g = (self.1 * scale).sqrt();
    let b = (self.2 * scale).sqrt();

    // Compute the translated [0,255] value of each color component.
    let ir = (256. * clamp(r, 0., 0.999)) as u8; // ir
    let ig = (256. * clamp(g, 0., 0.999)) as u8; // ig
    let ib = (256. * clamp(b, 0., 0.999)) as u8; // ib
    let ia = 255_u8;

    [ir, ig, ib, ia]
  }
}
