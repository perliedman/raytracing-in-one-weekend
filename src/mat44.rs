use std::ops::*;
use ::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Mat44([[f32; 4]; 4]);

impl Mat44 {
  fn translate(x: f32, y: f32, z: f32) {
    Mat44([
      [0.0, 0.0, 0.0, x],
      [0.0, 0.0, 0.0, y],
      [0.0, 0.0, 0.0, z],
      [0.0, 0.0, 0.0, 0.0]])
  }
}

impl Mul<Vec3> for Mat44 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 { e: [
      self.0[0][0] * other.e[0] + self.0[0][1] * other.e[1] + self.0[0][2] * other.e[2] + self.0[0][3],
      self.0[1][0] * other.e[0] + self.0[1][1] * other.e[1] + self.0[1][2] * other.e[2] + self.0[1][3],
      self.0[2][0] * other.e[0] + self.0[2][1] * other.e[1] + self.0[2][2] * other.e[2] + self.0[2][3]]
    }
  }
}

impl Mul for Mat44 {
  type Output = Mat44;

  fn mul(self, other: Mat44) -> Mat44 {
    let mut r = Mat44([[0.0; 4]; 4]);
    for i in 0..4 {
      for j in 0..4 {
        for k in 0..4 {
          r.0[i][j] += self.0[i][k] * other.0[k][j];
        }
      }
    }
    r
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test()]
  fn matrix_mul() {
    let a = Mat44([
      [1.0, 2.0, 3.0, 0.0],
      [4.0, 5.0, 6.0, 0.0],
      [7.0, 8.0, 9.0, 0.0],
      [0.0, 0.0, 0.0, 0.0]]);
    let b = Mat44([
      [9.0, 8.0, 7.0, 0.0],
      [6.0, 5.0, 4.0, 0.0],
      [3.0, 2.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 0.0]]);

    let expected = Mat44([
      [30.0, 24.0, 18.0, 0.0],
      [84.0, 69.0, 54.0, 0.0],
      [138.0, 114.0, 90.0, 0.0],
      [0.0, 0.0, 0.0, 0.0]]);

    let result = a * b;

    for i in 0..4 {
      for j in 0..4 {
        assert_eq!(expected.0[i][j], result.0[i][j]);
      }
    }
  }
}
