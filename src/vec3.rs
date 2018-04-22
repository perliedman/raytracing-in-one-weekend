use std::ops::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
  pub e: [f32; 3]
}


impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { e: [x, y, z] }
  }

  pub fn x(&self) -> f32 {
    self.e[0]
  }

  pub fn y(&self) -> f32 {
    self.e[1]
  }

  pub fn z(&self) -> f32 {
    self.e[2]
  }

  pub fn r(&self) -> f32 {
    self.e[0]
  }

  pub fn g(&self) -> f32 {
    self.e[1]
  }

  pub fn b(&self) -> f32 {
    self.e[2]
  }

  pub fn length(&self) -> f32 {
    (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
  }

  pub fn squared_length(&self) -> f32 {
    self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
  }

  pub fn dot(&self, v2: Vec3) -> f32 {
    self.e[0] * v2.e[0] + self.e[1] * v2.e[1] + self.e[2] * v2.e[2]
  }

  pub fn cross(&self, v2: Vec3) -> Vec3 {
    Vec3 { e: [
      self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
      -self.e[0] * v2.e[2] - self.e[2] * v2.e[0],
      self.e[0] * v2.e[1] - self.e[1] * v2.e[0]
    ]}
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
  }
}

impl Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
  }
}

impl Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
  }
}

impl Div for Vec3 {
  type Output = Vec3;

  fn div(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]] }
  }
}

impl Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Vec3 {
    Vec3 { e: [self * v.e[0], self * v.e[1], self * v.e[2]] }
  }
}

impl Div<f32> for Vec3 {
  type Output = Vec3;

  fn div(self, t: f32) -> Vec3 {
    Vec3 { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] }
  }
}

impl Mul<f32> for Vec3 {
  type Output = Vec3;

  fn mul(self, t: f32) -> Vec3 {
    Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
  }
}

impl Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Vec3 {
    Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Vec3) {
    self.e[0] += other.e[0];
    self.e[1] += other.e[1];
    self.e[2] += other.e[2];
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Vec3) {
    self.e[0] -= other.e[0];
    self.e[1] -= other.e[1];
    self.e[2] -= other.e[2];
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, other: Vec3) {
    self.e[0] *= other.e[0];
    self.e[1] *= other.e[1];
    self.e[2] *= other.e[2];
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, other: Vec3) {
    self.e[0] /= other.e[0];
    self.e[1] /= other.e[1];
    self.e[2] /= other.e[2];
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, t: f32) {
    self.e[0] *= t;
    self.e[1] *= t;
    self.e[2] *= t;
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, t: f32) {
    let k = 1.0 / t;

    self.e[0] *= k;
    self.e[1] *= k;
    self.e[2] *= k;
  }
}

impl Index<usize> for Vec3 {
  type Output = f32;

  fn index(&self, i: usize) -> &f32 {
    &self.e[i]
  }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  let mut r = Vec3 { e: [v.e[0], v.e[1], v.e[2]] };
  r /= v.length();
  r
}
