
use ::vec3::{Vec3};
use ::ray::Ray;

#[derive(Copy, Clone)]
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3
}

impl Aabb {
  pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
    for a in 0..3 {
      let mint = (self.min[a] - r.origin[a]) / r.direction[a];
      let maxt = (self.max[a] - r.origin[a]) / r.direction[a];
      let t0 = ffmin(mint, maxt);
      let t1 = ffmax(mint, maxt);

      tmin = ffmin(t0, tmin);
      tmax = ffmax(t1, tmax);

      if tmax <= tmin {
        return false;
      }
    }

    true
  }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
  let small = Vec3::new(
    ffmin(box0.min.x(), box1.min.x()),
    ffmin(box0.min.y(), box1.min.y()),
    ffmin(box0.min.z(), box1.min.z()));
  let big = Vec3::new(
    ffmax(box0.max.x(), box1.max.x()),
    ffmax(box0.max.y(), box1.max.y()),
    ffmax(box0.max.z(), box1.max.z()));

  Aabb { min: small, max: big }
}

fn ffmax(a: f32, b: f32) -> f32 {
  if a > b { a } else { b }
}

fn ffmin(a: f32, b: f32) -> f32 {
  if a < b { a } else { b }
}
