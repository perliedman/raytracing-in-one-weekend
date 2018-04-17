use ::vec3::Vec3;
use ::ray::Ray;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3
}

impl Camera {
  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
  }
}
