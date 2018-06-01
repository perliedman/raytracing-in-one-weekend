use vec3::*;
use ray::Ray;
use hitable::Hitable;
use bvh::BvhTree;

pub trait SceneEnvironment : Sync {
  fn color(&self, r: &Ray) -> Vec3;
}

pub struct SimpleSky { }

impl SceneEnvironment for SimpleSky {
  fn color(&self, r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
  }
}

pub struct Void { }

impl SceneEnvironment for Void {
  fn color(&self, _r: &Ray) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
  }
}

pub struct Scene<'a> {
  pub bvh: BvhTree<'a>,
  pub environment: Box<SceneEnvironment>,
  pub max_ray_depth: i32
}

impl<'a> Scene<'a> {
  pub fn new(models: &'a mut Vec<Box<Hitable>>, environment: Box<SceneEnvironment>, max_ray_depth: i32) -> Scene<'a> {
    Scene {
      bvh: BvhTree::new(models),
      environment,
      max_ray_depth
    }
  }
}