use ::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    Ray { origin, direction }
  }

  pub fn point_at_parameter(self, t: f32) -> Vec3 {
    self.origin + t * self.direction
  }
}
