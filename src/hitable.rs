extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::f32;

use ::vec3::{Vec3, unit_vector};
use ::ray::Ray;
use ::aabb::{Aabb, surrounding_box};

pub trait Hitable {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
  fn bounding_box(&self) -> Option<Aabb>;
}

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: &'a Material
}

impl<'a> Hitable for &'a [Box<Hitable>] {
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

  fn bounding_box(&self) -> Option<Aabb> {
    Some(Aabb {
      min: self.center - Vec3::new(self.radius, self.radius, self.radius),
      max: self.center + Vec3::new(self.radius, self.radius, self.radius),
    })
  }
}

pub struct BvhTree {
  nodes: Vec<BvhNode>
}

struct BvhNode {
  left: Option<NodeId>,
  right: Option<NodeId>,
  aabb: Aabb,
  hitable: Option<Box<Hitable>>
}

#[derive(Copy, Clone)]
struct NodeId {
  index: usize
}

// impl BvhNode {
//   pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
//     match self.hitable {
//       Some(hitable) => return hitable.hit(r, tmin, tmax),
//       None => return None
//     }
//   }
// }

impl BvhTree {
  fn hit(&self, id: NodeId, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let node = &self.nodes[id.index];

    if node.aabb.hit(r, tmin, tmax) {
      match node.hitable {
        Some(ref hitable) => return hitable.hit(r, tmin, tmax),
        None => { }
      }

      let mut hit_left: Option<HitRecord> = None;
      let mut hit_right: Option<HitRecord> = None;

      if let Some(ref left_index) = node.left {
        hit_left = self.hit(*left_index, r, tmin, tmax);
      }

      if let Some(ref right_index) = node.right {
        hit_right = self.hit(*right_index, r, tmin, tmax);
      }

      match hit_left {
        Some(left) => {
          match hit_right {
            Some(right) => if left.t < right.t { return hit_left; } else { return hit_right; },
            None => return hit_left
          }
        },
        None => {}
      }

      match hit_right {
        Some(_right) => return hit_right,
        None => {}
      }
    }

    None
  }
}

impl Hitable for BvhTree {
  fn bounding_box(&self) -> Option<Aabb> {
    Some(self.nodes[0].aabb)
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    self.hit(NodeId { index: 0 }, r, tmin, tmax)
  }
}

impl BvhTree {
  fn new(&mut self, aabb: Aabb, hitable: Option<Box<Hitable>>) -> NodeId {
    let next_index = self.nodes.len();

    self.nodes.push(BvhNode {
      left: None,
      right: None,
      aabb,
      hitable
    });

    return NodeId { index: next_index };
  }
}

// impl<'a> BvhNode<'a> {
//   pub fn new(l: &'static mut [Box<Hitable>]) -> BvhNode<'static> {
//     let axis = rand::thread_rng().gen_range::<i32>(0, 3);

//     match axis {
//       0 => l.sort_by(|a, b| box_x_compare(a, b)),
//       1 => l.sort_by(|a, b| box_y_compare(a, b)),
//       2 => l.sort_by(|a, b| box_z_compare(a, b)),
//       _ => panic!("Unexpected axis")
//     }

//     let left: &Box<Hitable>;
//     let right: &Box<Hitable>;
//     let left_hitables: &mut [Box<Hitable>];
//     let right_hitables: &mut [Box<Hitable>];
//     let halfLen = l.len() / 2;

//     if l.len() == 1 {
//       left = &l[0];
//       right = &l[0];
//     } else if l.len() == 2 {
//       left = &l[0];
//       right = &l[1];
//     } else {
//       left_hitables = &mut l[0..halfLen];
//       left = &Box::new(BvhNode::new(left_hitables));
//       right_hitables = &mut l[halfLen..];
//       right = &Box::new(BvhNode::new(right_hitables));
//     }

//     if let Some(left_box) = left.bounding_box() {
//       if let Some(right_box) = right.bounding_box() {
//         return BvhNode { left: &left, right: &right, aabb: surrounding_box(&left_box, &right_box) };
//       }
//     }

//     panic!("No bounding box in BvhNode::new");
//  } 
// }

fn box_x_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.x().partial_cmp(&box_right.min.x()) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.y().partial_cmp(&box_right.min.y()) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.z().partial_cmp(&box_right.min.z()) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}


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
