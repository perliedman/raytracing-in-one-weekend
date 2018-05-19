extern crate rayon;
extern crate rand;
extern crate indicatif;

use renderer::rayon::prelude::*;
use std::f32;
use self::indicatif::{ProgressBar, ProgressStyle};

use ::vec3::{Vec3, unit_vector};
use ray::Ray;
use hitable::*;
use camera::Camera;


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
  pub model: &'a Hitable,
  pub environment: Box<SceneEnvironment>
}

pub fn render(scene: &Scene, camera: &Camera, nx: usize, ny: usize, ns: usize) -> Vec<u8> {
  let bar = &Box::new(ProgressBar::new((nx * ny / 64) as u64));
  bar.set_prefix("🎨  Rendering");
  bar.set_style(ProgressStyle::default_bar()
    .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%"));

  let pixels = (0..ny).into_par_iter().rev().flat_map(|j| (0..nx).into_par_iter().flat_map(move |i| {
    let mut col = Vec3::new(0.0, 0.0, 0.0);
    for _s in 0..ns {
      let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
      let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);

      let r = camera.get_ray(u, v);
      col += color(&r, *&scene, 0);
    }

    if i % 64 == 0 {
      bar.inc(1);
    }

    col /= ns as f32;
    col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
    (0..3).into_par_iter().map(move |k| (255.99 * col[k as usize]).min(255.0) as u8)
  })).collect();

  // let pixels: Vec<u8> = (0..ny).into_par_iter().rev().flat_map(|j| {
  //   (0..nx).into_par_iter().flat_map(move |i| {
  //     let col = Vec3::new(0.0, 0.0, 0.0);
      // let mut col = Vec3::new(0.0, 0.0, 0.0);

      // for _s in 0..ns {
      //   let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
      //   let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);

      //   let r = camera.get_ray(u, v);
      //   col += color(&r, *&scene, 0);
      // }

      // col /= ns as f32;
      // col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

      // // c += 1;
      // // if c % 50 == 0 {
      // //   bar.inc(50);
      // // }

  //     (0..3).into_par_iter().map(move |k| (255.99 * col[k]).min(255.0) as u8)
  //   })
  // }).collect();

  bar.finish();

  pixels
}

fn color(r: &Ray, scene: &Scene, depth: i32) -> Vec3 {
  let hit = scene.model.hit(&r, 0.001, f32::MAX);

  match hit {
    Some(rec) => {
      let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
      if depth < 50 {
        match rec.material.scatter(&r, &rec) {
          Some(scatter) => {
            if let Some(bounce) = scatter.ray {
              return emitted + scatter.color * color(&bounce, scene, depth + 1)
            }
          },
          None => {}
        }
      }

      return emitted;
    },
    None => return scene.environment.color(&r)
  }
}
