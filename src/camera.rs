extern crate rand;

use std::f32;
use ::vec3::*;
use ::ray::Ray;

#[derive(Debug)]
pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub lens_radius: f32,
  u: Vec3,
  v: Vec3,
  w: Vec3
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
    let theta = vfov * f32::consts::PI / 180.0;
    let half_height = f32::tan(theta / 2.0);
    let half_width = aspect * half_height;    

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(vup.cross(w));
    let v = w.cross(u);

    eprintln!("{:?}", w);
    eprintln!("{:?}", u);
    eprintln!("{:?}", v);

    Camera {
      lower_left_corner: lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
      horizontal: 2.0 * half_width * focus_dist * u,
      vertical: 2.0 * half_height * focus_dist * v,
      origin: lookfrom,
      lens_radius: aperture / 2.0,
      u,
      v,
      w
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = rd.x() * self.u + rd.y() * self.v;

    Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
  }
}

fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    if p.dot(p) < 1.0 {
      return p;
    }
  }
}
