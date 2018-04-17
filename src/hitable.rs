use ::vec3::Vec3;
use ::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3
}

pub trait Hitable {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<Hitable>> {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let mut hit: Option<HitRecord> = None;
    let closestSoFar = tmax;

    for hitable in self {
      if let Some(candidateHit) = hitable.hit(r, tmin, tmax) {
        match hit {
          None => hit = Some(candidateHit),
          Some(prev) => if candidateHit.t < prev.t {
            hit = Some(candidateHit);
          }
        }
      }
    }

    hit
  }
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.dot(r.direction);
    let b = oc.dot(r.direction);
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;

    if discriminant > 0. {
      let t = (-b - discriminant.sqrt()) / a;

      if t < tmax && t > tmin {
        let p = r.point_at_parameter(t);

        return Some(HitRecord {
          t,
          p,
          normal: (p - self.center) / self.radius
        });
      }
    }

    None
  } 
}