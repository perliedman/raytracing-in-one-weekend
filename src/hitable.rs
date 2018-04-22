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
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
      let target = rec.p + rec.normal + random_in_unit_sphere();
      return Some(Scatter {
        color: self.albedo,
        ray: Some(Ray::new(rec.p, target - rec.p))
      });
  }
}

pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f32
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
    let reflected = reflect(&r_in.direction, &rec.normal);
    let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
    return Some(Scatter {
      color: self.albedo,
      ray: if scattered.direction.dot(rec.normal) > 0.0 { Some(scattered) } else { None }
    })
  }
}

pub struct Dielectric {
  pub ref_idx: f32
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
    let outward_normal: Vec3;
    let ni_over_nt: f32;
    let cosine: f32;
    
    if r_in.direction.dot(rec.normal) > 0.0 {
      outward_normal = -rec.normal;
      ni_over_nt = self.ref_idx;
      cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.length();
    } else {
      outward_normal = rec.normal;
      ni_over_nt = 1.0 / self.ref_idx;
      cosine = -r_in.direction.dot(rec.normal) / r_in.direction.length();
    }

    let albedo = Vec3::new(1.0, 1.0, 1.0);

    match refract(&r_in.direction, &outward_normal, ni_over_nt) {
      Some(refraction) => { 
        eprintln!("refraction");
        if rand::random::<f32>() > schlick(cosine, self.ref_idx) {
          return Some(Scatter { color: albedo, ray: Some(Ray::new(rec.p, refraction)) });
        }
      },
      None => { }
    }
    
    eprintln!("reflection");
    Some(Scatter { color: albedo, ray: Some(Ray::new(rec.p, reflect(&unit_vector(r_in.direction), &rec.normal))) })
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

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
  let uv = unit_vector(*v);
  let dt = uv.dot(*n);
  let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    Some(ni_over_nt * (uv - *n * dt) - discriminant.sqrt() * *n)
  } else {
    None
  }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  let r0sq = r0 * r0;
  r0sq + (1.0 - r0sq) * (1.0 - cosine).powf(5.0)
}
