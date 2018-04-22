use std::f32;
use ::vec3::*;
use ::ray::Ray;

#[derive(Debug)]
pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
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
      lower_left_corner: lookfrom - half_width * u - half_height * v - w,
      horizontal: 2.0 * half_width * u,
      vertical: 2.0 * half_height * v,
      origin: lookfrom
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
  }
}
