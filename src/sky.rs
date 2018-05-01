/*
  This is more or less a straight port of the codeb by
  Nico Schertler found on
  https://nicoschertler.wordpress.com/2013/04/03/simulating-a-days-sky/

  License unknown, but I assume it's free to use.

  The code is based on the paper "A Practical Analytic Model for Daylight"
  by A. J. Preetham, Peter Shirley, Brian Smits
  http://www.cs.utah.edu/~shirley/papers/sunsky/sunsky.pdf
*/

use std::f32::consts::PI;
use std::f32;

use ::vec3::Vec3;

pub struct Sky {
  turbidity: f32,
  solar_zenith: f32,
  solar_azimuth: f32,
  y_z: f32,
  xz: f32,
  yz: f32,
  coeffs: Coeffs
}

struct Coeffs {
  coeffs_y: Coeff,
  coeffsx: Coeff,
  coeffsy: Coeff
}

struct Coeff { a: f32, b: f32, c: f32, d: f32, e: f32 }

impl Coeff {
  fn zero() -> Coeff { Coeff { a: 0.0, b: 0.0, c: 0.0, d: 0.0, e: 0.0 } }
}

impl Sky {
  pub fn new(turbidity: f32, solar_zenith: f32, solar_azimuth: f32) -> Sky {
    let mut sky = Sky { 
      turbidity, 
      solar_zenith, 
      solar_azimuth, 
      y_z: 0.0, 
      xz: 0.0, 
      yz: 0.0, 
      coeffs: Coeffs { coeffs_y: Coeff::zero(), coeffsx: Coeff::zero(), coeffsy: Coeff::zero() } };
    sky.calculate_zenital_absolutes();
    sky.calculate_coefficents();

    sky
  }

  fn calculate_zenital_absolutes(&mut self) {
    let turbidity = self.turbidity;
    let solar_zenith = self.solar_zenith;

    let y_z = self.y_(PI - 2.0 * solar_zenith);
    let y0 = self.y_(PI);
    self.y_z = y_z / y0;

    let z3 = solar_zenith.powi(3);
    let z2 = solar_zenith * solar_zenith;
    let z = solar_zenith;
    let t_vec = vec![turbidity * turbidity, turbidity, 1.0];
    let z_vec = vec![z3, z2, z, 1.0];
 
    let x = mul(&vec![
        vec![0.00166, -0.00375, 0.00209, 0.0],
        vec![-0.02903, 0.06377, -0.03202, 0.00394],
        vec![0.11693, -0.21196, 0.06052, 0.25886],
        vec![0.0, 0.0, 0.0, 0.0]
      ],
      &z_vec);
    let xz = dot(&t_vec, &x);
    self.xz = xz;
 
    let y = mul(&vec![
        vec![0.00275, -0.00610, 0.00317, 0.0],
        vec![-0.04214, 0.08970, -0.04153, 0.00516],
        vec![0.15346, -0.26756, 0.06670, 0.26688],
        vec![0.0, 0.0, 0.0, 0.0]
      ],
      &z_vec);
    let yz = dot(&t_vec, &y);
    self.yz = yz;
  }

  fn calculate_coefficents(&mut self) {
    let turbidity = self.turbidity;
    self.coeffs = Coeffs {
      coeffs_y: Coeff {
        a: 0.1787 * turbidity - 1.4630,
        b: -0.3554 * turbidity + 0.4275,
        c: -0.0227 * turbidity + 5.3251,
        d: 0.1206 * turbidity - 2.5771,
        e: -0.0670 * turbidity + 0.3703,
      },
      coeffsx: Coeff {
        a: -0.0193 * turbidity - 0.2592,
        b: -0.0665 * turbidity + 0.0008,
        c: -0.0004 * turbidity + 0.2125,
        d: -0.0641 * turbidity - 0.8989,
        e: -0.0033 * turbidity + 0.0452,
      },
      coeffsy: Coeff {
        a: -0.0167 * turbidity - 0.2608,
        b: -0.0950 * turbidity + 0.0092,
        c: -0.0079 * turbidity + 0.2102,
        d: -0.0441 * turbidity - 1.6537,
        e: -0.0109 * turbidity + 0.0529,
      }
    }
  }

  fn perez(&self, zenith: f32, gamma: f32, coeffs: &Coeff) -> f32 {
    (1.0 + coeffs.a * (coeffs.b/zenith.cos()).exp()) *
      (1.0 + coeffs.c*(coeffs.d*gamma).exp()+coeffs.e*(gamma.cos().powi(2)))
  }

  fn cie_to_rgb(y_: f32, x: f32, y: f32) -> Vec3 {
    let x_ = x / y * y_;
    let z_ = (1.0 - x - y) / y * y_;
    let m = mul(&vec![
        vec![3.2406, - 1.5372, -0.4986, 0.0],
        vec![-0.9689, 1.8758, 0.0415, 0.0],
        vec![0.0557, -0.2040, 1.0570, 0.0],
        vec![0.0, 0.0, 0.0, 1.0]
      ],
      &vec![x_, y_, z_, 1.0]);
    Vec3::new(m[0], m[1], m[2])
  }

  fn gamma(&self, zenith: f32, azimuth: f32) -> f32 {
    let solar_zenith = self.solar_zenith;
    (solar_zenith.sin()*zenith.sin()*(azimuth-self.solar_azimuth).cos()+solar_zenith.cos()*zenith.cos()).acos()
  }

  pub fn rgb(&self, azimuth: f32, mut zenith: f32) -> Vec3 {
    let solar_zenith = self.solar_zenith;

    let g = self.gamma(zenith, azimuth);
    zenith = zenith.min(PI/2.0).max(-PI/0.5);
    let y_p = self.y_z * self.perez(zenith, g, &self.coeffs.coeffs_y) / self.perez(0.0, solar_zenith, &self.coeffs.coeffs_y);
    let xp = self.xz * self.perez(zenith, g, &self.coeffs.coeffsx) / self.perez(0.0, solar_zenith, &self.coeffs.coeffsx);
    let yp = self.yz * self.perez(zenith, g, &self.coeffs.coeffsy) / self.perez(0.0, solar_zenith, &self.coeffs.coeffsy);

    Sky::cie_to_rgb(y_p, xp, yp)
  }

  fn y_(&self, e: f32) -> f32 {
    (4.0453 * self.turbidity - 4.9710) * ((4.0 / 9.0 - self.turbidity / 120.0) * e).tan() - 0.2155 * self.turbidity + 2.4192
  }
}

fn mul(m: &Vec<Vec<f32>>, v: &Vec<f32>) -> Vec<f32> {
  m.into_iter().map(|r| dot(r, v)).collect()
}

fn dot(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
  v1.into_iter().zip(v2.into_iter()).fold(0.0, |a, (x, y)| a + x * y)
}
