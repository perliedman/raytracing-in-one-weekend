use std::f32;
use std::ops::*;
use ::vec3::{Vec3, unit_vector};

#[derive(Copy, Clone, Debug)]
pub struct Mat44([[f32; 4]; 4]);

impl Mat44 {
  pub fn identity() -> Mat44 {
    Mat44([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn translate(displacement: Vec3) -> Mat44 {
    Mat44([
      [1.0, 0.0, 0.0, -displacement.x()],
      [0.0, 1.0, 0.0, -displacement.y()],
      [0.0, 0.0, 1.0, -displacement.z()],
      [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn rotate(angle: f32, axis: Vec3) -> Mat44 {
    let ax = unit_vector(axis);
    let radians = angle * f32::consts::PI / 180.0;
    let s = radians.sin();
    let c = radians.cos();
    let t = 1.0 - c;

    Mat44([
      [t * ax[0] * ax[0] + c, t * ax[0] * ax[1] + s * ax[2], t * ax[0] * ax[2] - s * ax[1], 0.0],
      [t * ax[1] * ax[0] - s * ax[2], t * ax[1] * ax[1] + c, t * ax[1] * ax[2] + s * ax[0], 0.0],
      [t * ax[2] * ax[0] + s * ax[1], t * ax[2] * ax[1] - s * ax[0], t * ax[2] * ax[2] + c, 0.0],
      [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn inverse(&self) -> Mat44 {
    let mut dst = Mat44::identity();
    let mut tmp = self.clone();

    for i in 0..4 {
      let mut val = tmp.0[i][i];
      let mut ind = i;
      for j in i+1..4 {
        if tmp.0[i][j].abs() > val.abs() {
          ind = j;
          val = tmp.0[i][j];
        }
      }

      if ind != i {
        // Swap columns
        for j in 0..4 {
          dst.0[j].swap(i, ind);
          tmp.0[j].swap(i, ind);
        }
      }

      if val.abs() < 1e-6 {
        panic!("Singular matrix, no inverse");
      }

      let ival = 1.0 / val;
      for j in 0..4 {
        tmp.0[j][i] *= ival;
        dst.0[j][i] *= ival;
      }

      for j in 0..4 {
        if j == i { continue; }

        val = tmp.0[i][j];
        for k in 0..4 {
          tmp.0[k][j] -= tmp.0[k][i] * val;
          dst.0[k][j] -= dst.0[k][i] * val;
        }        
      }
    }

    dst
  }

  pub fn mul_as_33(&self, other: Vec3) -> Vec3 {
    Vec3 { e: [
      self.0[0][0] * other.e[0] + self.0[0][1] * other.e[1] + self.0[0][2] * other.e[2],
      self.0[1][0] * other.e[0] + self.0[1][1] * other.e[1] + self.0[1][2] * other.e[2],
      self.0[2][0] * other.e[0] + self.0[2][1] * other.e[1] + self.0[2][2] * other.e[2]]
    }
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
