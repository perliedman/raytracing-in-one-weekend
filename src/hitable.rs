use std::f32;
use std::fmt;
use std::rc::Rc;

use ::material::{Material, HitRecord};

use ::vec3::{Vec3};
use ::ray::Ray;
use ::aabb::{Aabb, surrounding_box};

pub trait Hitable {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
  fn bounding_box(&self) -> Option<Aabb>;
}

impl fmt::Debug for Hitable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Hitable {{ aabb: {:?} }}", self.bounding_box())
  }
}

impl<'a> Hitable for Vec<Box<Hitable>> {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let mut hit: Option<HitRecord> = None;

    for hitable in self.iter() {
      if let Some(candidate_hit) = hitable.hit(r, tmin, tmax) {
        match hit {
          None => hit = Some(candidate_hit),
          Some(prev) => if candidate_hit.t < prev.t {
            hit = Some(candidate_hit);
          }
        }
      }
    }

    hit
  }

  fn bounding_box(&self) -> Option<Aabb> {
    if self.len() < 1 { 
      return None;
    }

    let mut result: Aabb;
    let first = self[0].bounding_box();
    match first {
      Some(b) => result = b,
      None => return None
    }

    for i in 1..self.len() {
      if let Some(b) = self[i].bounding_box() {
        result = surrounding_box(&result, &b);
      } else {
        return None;
      }
    }

    Some(result)
  }
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: Rc<Material>
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
          normal: (p - self.center) / self.radius,
          material: &*self.material,
          // TODO: texture coords
          u: 0.0,
          v: 0.0
        });
      }
    }

    None
  }

  fn bounding_box(&self) -> Option<Aabb> {
    Some(Aabb {
      min: self.center - Vec3::new(self.radius, self.radius, self.radius),
      max: self.center + Vec3::new(self.radius, self.radius, self.radius),
    })
  }
}

pub struct XyRect {
  pub x0: f32,
  pub x1: f32,
  pub y0: f32,
  pub y1: f32,
  pub k: f32,
  pub material: Rc<Material>
}

impl Hitable for XyRect {
  fn bounding_box(&self) -> Option<Aabb> {
    Some(Aabb {
      min: Vec3::new(self.x0, self.y0, self.k - 1e-4),
      max: Vec3::new(self.x1, self.y1, self.k + 1e-4)
    })
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let t = (self.k - r.origin.z()) / r.direction.z();

    if t < tmin || t > tmax { return None; }

    let x = r.origin.x() + t * r.direction.x();
    let y = r.origin.y() + t * r.direction.y();

    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
      return None;
    }

    return Some(HitRecord {
      u: (x - self.x0) / (self.x1 - self.x0),
      v: (y - self.y0) / (self.y1 - self.y0),
      t,
      material: &*self.material,
      p: r.point_at_parameter(t),
      normal: Vec3::new(0.0, 0.0, 1.0)
    })
  }
}

pub struct XzRect {
  pub x0: f32,
  pub x1: f32,
  pub z0: f32,
  pub z1: f32,
  pub k: f32,
  pub material: Rc<Material>
}

impl Hitable for XzRect {
  fn bounding_box(&self) -> Option<Aabb> {
    Some(Aabb {
      min: Vec3::new(self.x0, self.k - 1e-4, self.z0),
      max: Vec3::new(self.x1, self.k + 1e-4, self.z1)
    })
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let t = (self.k - r.origin.y()) / r.direction.y();

    if t < tmin || t > tmax { return None; }

    let x = r.origin.x() + t * r.direction.x();
    let z = r.origin.z() + t * r.direction.z();

    if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
      return None;
    }

    return Some(HitRecord {
      u: (x - self.x0) / (self.x1 - self.x0),
      v: (z - self.z0) / (self.z1 - self.z0),
      t,
      material: &*self.material,
      p: r.point_at_parameter(t),
      normal: Vec3::new(0.0, 0.0, 1.0)
    })
  }
}

pub struct YzRect {
  pub y0: f32,
  pub y1: f32,
  pub z0: f32,
  pub z1: f32,
  pub k: f32,
  pub material: Rc<Material>
}

impl Hitable for YzRect {
  fn bounding_box(&self) -> Option<Aabb> {
    Some(Aabb {
      min: Vec3::new(self.k - 1e-4, self.y0, self.z0),
      max: Vec3::new(self.k + 1e-4, self.y1, self.z1)
    })
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let t = (self.k - r.origin.x()) / r.direction.x();

    if t < tmin || t > tmax { return None; }

    let y = r.origin.y() + t * r.direction.y();
    let z = r.origin.z() + t * r.direction.z();

    if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
      return None;
    }

    return Some(HitRecord {
      u: (y - self.y0) / (self.y1 - self.y0),
      v: (z - self.z0) / (self.z1 - self.z0),
      t,
      material: &*self.material,
      p: r.point_at_parameter(t),
      normal: Vec3::new(0.0, 0.0, 1.0)
    })
  }
}

pub struct FlipNormals {
  pub hitable: Box<Hitable>
}

impl Hitable for FlipNormals {
  fn bounding_box(&self) -> Option<Aabb> {
    self.hitable.bounding_box()
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    match self.hitable.hit(r, tmin, tmax) {
      Some(mut hit) => {
        hit.normal = -hit.normal;
        return Some(hit);
      },
      None => None
    }
  }
}
