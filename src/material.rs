extern crate rand;

use ::vec3::{Vec3, unit_vector};
use ::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: &'a Material,
  pub u: f32,
  pub v: f32
}

pub struct Scatter {
  pub color: Vec3,
  pub ray: Option<Ray>
}

pub trait Material : Sync + Send {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
  fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
  }
}

pub trait Texture : Sync + Send {
  fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
  color: Vec3
}

impl ConstantTexture {
  pub fn new(r: f32, g: f32, b: f32) -> ConstantTexture {
    ConstantTexture { color: Vec3::new(r, g, b) }
  }
}

impl Texture for ConstantTexture {
  fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
    self.color
  }
}

pub struct CheckerTexture {
  pub odd: Box<Texture>,
  pub even: Box<Texture>
}

impl Texture for CheckerTexture {
  fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
    let sines = 
      (p.x()*10.0).sin() *
      (p.y()*10.0).sin() *
      (p.z()*10.0).sin();

    if sines < 0.0 {
      self.odd.value(u, v, p)
    } else {
      self.even.value(u, v, p)
    }
  }
}

pub struct Lambertian {
  pub albedo: Box<Texture>
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
      let target = rec.p + rec.normal + random_in_unit_sphere();
      return Some(Scatter {
        color: self.albedo.value(0.0, 0.0, &rec.p),
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
        // eprintln!("refraction");
        if rand::random::<f32>() > schlick(cosine, self.ref_idx) {
          return Some(Scatter { color: albedo, ray: Some(Ray::new(rec.p, refraction)) });
        }
      },
      None => { }
    }
    
    // eprintln!("reflection");
    Some(Scatter { color: albedo, ray: Some(Ray::new(rec.p, reflect(&unit_vector(r_in.direction), &rec.normal))) })
  }
}

pub struct DiffuseLight {
  pub emit: Box<Texture>
}

impl Material for DiffuseLight {
  fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Scatter> {
    None
  }

  fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
    self.emit.value(u, v, p)
  }
}

pub struct Isotropic {
  pub albedo: Box<Texture>
}

impl Material for Isotropic {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
    Some(Scatter {
      color: self.albedo.value(rec.u, rec.v, &rec.p),
      ray: Some(Ray::new(rec.p, random_in_unit_sphere()))
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
