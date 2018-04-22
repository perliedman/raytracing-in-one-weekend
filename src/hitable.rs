extern crate rand;

use ::vec3::{Vec3, unit_vector};
use ::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: &'a Material
}

pub trait Hitable {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<Hitable>> {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let mut hit: Option<HitRecord> = None;

    for hitable in self {
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
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: Box<Material>
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
          material: &*self.material
        });
      }
    }

    None
  } 
}

#[derive(Clone, Copy)]
pub struct Scatter {
  pub color: Vec3,
  pub ray: Option<Ray>
}

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
  pub albedo: Vec3
}

impl Material for Lambertian {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
      let target = rec.p + rec.normal + random_in_unit_sphere();
      return Some(Scatter {
        color: self.albedo,
        ray: Some(Ray::new(rec.p, target - rec.p))
      });
  }
}

pub struct Metal {
  pub albedo: Vec3
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
    let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
    let scattered = Ray::new(rec.p, reflected);
    return Some(Scatter {
      color: self.albedo,
      ray: if scattered.direction.dot(rec.normal) > 0.0 { Some(scattered) } else { None }
    })
  }
}

fn random_in_unit_sphere() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
    if p.squared_length() <= 1.0 {
      return p;
    }
  }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  *v - 2.0 * v.dot(*n) * *n
}
