extern crate rand;
extern crate tobj;

pub mod triangle;

use std::f32;
use std::fmt;
use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;

use ::material::{Material, HitRecord, Isotropic, Texture};

use ::vec3::{Vec3};
use ::mat44::Mat44;
use ::ray::Ray;
use ::aabb::{Aabb, surrounding_box};
use ::material::*;
use triangle::Triangle;

pub trait Hitable : Sync {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
  fn bounding_box(&self) -> Option<Aabb>;
}

impl fmt::Debug for Hitable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Hitable {{ aabb: {:?} }}", self.bounding_box())
  }
}

impl Hitable for Vec<Box<Hitable>> {
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
  pub material: Arc<Material>
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.dot(r.direction);
    let b = oc.dot(r.direction);
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;

    if discriminant > 0. {
      let mut t = (-b - discriminant.sqrt()) / a;

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

      t = (-b + discriminant.sqrt()) / a;
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
  pub material: Arc<Material>
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
  pub material: Arc<Material>
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
      normal: Vec3::new(0.0, 1.0, 0.0)
    })
  }
}

pub struct YzRect {
  pub y0: f32,
  pub y1: f32,
  pub z0: f32,
  pub z1: f32,
  pub k: f32,
  pub material: Arc<Material>
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
      normal: Vec3::new(1.0, 0.0, 0.0)
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

pub fn new_box(p0: Vec3, p1: Vec3, material: Arc<Material>) -> Vec<Box<Hitable>> {
  vec![
    Box::new(XyRect { x0: p0.x(), x1: p1.x(), y0: p0.y(), y1: p1.y(), k: p1.z(), material: Arc::clone(&material)}),
    Box::new(FlipNormals { hitable: Box::new(XyRect { x0: p0.x(), x1: p1.x(), y0: p0.y(), y1: p1.y(), k: p0.z(), material: Arc::clone(&material)}) }),
    Box::new(XzRect { x0: p0.x(), x1: p1.x(), z0: p0.z(), z1: p1.z(), k: p1.y(), material: Arc::clone(&material)}),
    Box::new(FlipNormals { hitable: Box::new(XzRect { x0: p0.x(), x1: p1.x(), z0: p0.z(), z1: p1.z(), k: p0.y(), material: Arc::clone(&material)}) }),
    Box::new(YzRect { y0: p0.y(), y1: p1.y(), z0: p0.z(), z1: p1.z(), k: p1.x(), material: Arc::clone(&material)}),
    Box::new(FlipNormals { hitable: Box::new(YzRect { y0: p0.y(), y1: p1.y(), z0: p0.z(), z1: p1.z(), k: p0.x(), material: Arc::clone(&material)}) }),
  ]
}

pub struct Transform {
  pub hitable: Box<Hitable>,
  pub transform: Mat44,
  inverse_transform: Mat44
}

impl Transform {
  pub fn new(hitable: Box<Hitable>, transform: Mat44) -> Transform {
    Transform {
      hitable,
      transform,
      inverse_transform: transform.inverse()
    }
  }
}

impl Hitable for Transform {
  fn bounding_box(&self) -> Option<Aabb> {
    match self.hitable.bounding_box() {
      Some(bbox) => {
        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for i in 0..2 {
          for j in 0..2 {
            for k in 0..2 {
              let v = self.inverse_transform * Vec3::new(
                (i as f32) * bbox.max.x() + ((1 - i) as f32) * bbox.min.x(),
                (j as f32) * bbox.max.y() + ((1 - j) as f32) * bbox.min.y(),
                (k as f32) * bbox.max.z() + ((1 - k) as f32) * bbox.min.z());

              for c in 0..3 {
                if v[c] < min[c] {
                  min[c] = v[c];
                }
                if v[c] > max[c] {
                  max[c] = v[c];
                }
              }
            }
          }
        }

        return Some(Aabb { min, max })
      },
      None => None
    }
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let transformed_r = Ray {
      origin: self.transform * r.origin, 
      direction: self.transform.mul_as_33(r.direction)
    };

    match self.hitable.hit(&transformed_r, tmin, tmax) {
      Some(mut hit) => {
        hit.p = self.inverse_transform * hit.p;
        hit.normal = self.inverse_transform.mul_as_33(hit.normal);
        Some(hit)
      },
      None => None
    }
  }
}

pub struct ConstantMedium {
  boundary: Box<Hitable>,
  density: f32,
  phase_function: Isotropic
}

impl ConstantMedium {
  pub fn new(boundary: Box<Hitable>, density: f32, a: Box<Texture>) -> ConstantMedium {
    ConstantMedium {
      boundary,
      density,
      phase_function: Isotropic { albedo: a }
    }
  }
}

impl Hitable for ConstantMedium {
  fn bounding_box(&self) -> Option<Aabb> {
    self.boundary.bounding_box()
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    if let Some(mut hit1) = self.boundary.hit(r, -(f32::MAX), f32::MAX) {
      if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 1e-4, f32::MAX) {
        if hit1.t < tmin {
          hit1.t = tmin;
        }

        if hit2.t > tmax {
          hit2.t = tmax;
        }

        if hit1.t > hit2.t {
          return None;
        }

        if hit1.t < 0.0 {
          hit1.t = 0.0;
        }

        let distance_inside_boundary = (hit2.t - hit1.t) * r.direction.length();
        let hit_distance = (-1.0 / self.density) * rand::random::<f32>().ln();

        if hit_distance < distance_inside_boundary {
          let t = hit1.t + hit_distance / r.direction.length();
          return Some(HitRecord {
            t,
            p: r.point_at_parameter(t),
            normal: Vec3::new(1.0, 0.0, 0.0), // Arbitrary
            material: &self.phase_function,
            u: 0.0,
            v: 0.0
          })
        }
      }
    }

    None
  }
}

pub fn obj_to_hitable(path: &Path) -> Vec<Box<Hitable>> {
  let obj = self::tobj::load_obj(path);
  let (models, mtls) = obj.unwrap();
  let mut world: Vec<Box<Hitable>> = Vec::new();

  let default_mat: Arc<Material> = Arc::new(Lambertian { albedo: Box::new(ConstantTexture::new(0.6, 0.6, 0.6)) });
  let materials: Vec<Arc<Material>> = mtls.iter().map(|m| {
    let mat: Arc<Material> = match m.illumination_model {
      Some(7) => Arc::new(Dielectric { ref_idx: m.optical_density }),
      Some(5) => Arc::new(Metal { albedo: Vec3::new(m.diffuse[0], m.diffuse[1], m.diffuse[2]), fuzz: 1. / m.shininess }),
      _ => Arc::new(Lambertian { albedo: Box::new(ConstantTexture::new(m.diffuse[0], m.diffuse[1], m.diffuse[2])) })
    };

    mat
  }).collect();

  for m in models.iter() {
    let mesh = &m.mesh;
    for f in 0..mesh.indices.len() / 3 {
      let i0 = mesh.indices[3 * f] as usize;
      let i1 = mesh.indices[3 * f + 1] as usize;
      let i2 = mesh.indices[3 * f + 2] as usize;
      let v0 = Vec3::new(mesh.positions[i0 * 3], mesh.positions[i0 * 3 + 1], mesh.positions[i0 * 3 + 2]);
      let v1 = Vec3::new(mesh.positions[i1 * 3], mesh.positions[i1 * 3 + 1], mesh.positions[i1 * 3 + 2]);
      let v2 = Vec3::new(mesh.positions[i2 * 3], mesh.positions[i2 * 3 + 1], mesh.positions[i2 * 3 + 2]);

      let mat: Arc<Material> = match mesh.material_id {
        Some(id) => Arc::clone(&materials[id]),
        None => Arc::clone(&default_mat)
      };

      let tri: Triangle;
      if mesh.normals.len() > 0 {
        let normal = Vec3::new(mesh.normals[i0 * 3], mesh.normals[i0 * 3 + 1], mesh.normals[i0 * 3 + 2]);
        tri = Triangle::new_with_normal(v0, v1, v2, normal, mat)
      } else {
        tri = Triangle::new(v0, v1, v2, Arc::clone(&mat));
      }

      world.push(Box::new(tri));
    }
  }

  world
}
