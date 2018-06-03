use std::sync::Arc;

use ::material::*;
use ::vec3::{Vec3};
use ::ray::Ray;
use ::aabb::Aabb;
use ::geometry::*;

pub struct Triangle {
  v0: Vec3,
  v1: Vec3,
  v2: Vec3,
  normal: Vec3,
  material: Arc<Material>
}

impl Triangle {
  pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Arc<Material>) -> Triangle {
    Triangle { v0, v1, v2, normal: (v1 - v0).cross(v2 - v0), material }
  }

  pub fn new_with_normal(v0: Vec3, v1: Vec3, v2: Vec3, normal: Vec3, material: Arc<Material>) -> Triangle {
    Triangle { v0, v1, v2, normal, material }
  }
}

impl Hitable for Triangle {
  fn bounding_box(&self) -> Option<Aabb> {
    Some(Aabb {
      min: Vec3::new(
        self.v0.x().min(self.v1.x().min(self.v2.x())),
        self.v0.y().min(self.v1.y().min(self.v2.y())),
        self.v0.z().min(self.v1.z().min(self.v2.z()))),
      max: Vec3::new(
        self.v0.x().max(self.v1.x().max(self.v2.x())),
        self.v0.y().max(self.v1.y().max(self.v2.y())),
        self.v0.z().max(self.v1.z().max(self.v2.z()))),
    })
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let v0v1 = self.v1 - self.v0;
    let v0v2 = self.v2 - self.v0;
    let pvec = r.direction.cross(v0v2);
    let det = v0v1.dot(pvec);

    if det.abs() < 1e-4 {
      return None
    }
    let inv_det = 1. / det;

    let tvec = r.origin - self.v0;
    let u = tvec.dot(pvec) * inv_det;
    if u < 0. || u > 1. {
      return None
    }

    let qvec = tvec.cross(v0v1);
    let v = r.direction.dot(qvec) * inv_det;
    if v < 0. || u + v > 1. {
      return None
    }

    let t = v0v2.dot(qvec) * inv_det;

    if t < tmin || t > tmax {
      return None
    }

    let p = r.point_at_parameter(t);

    return Some(HitRecord {
      u,
      v,
      t,
      p,
      normal: self.normal,
      material: &*self.material
    })
  }
}
