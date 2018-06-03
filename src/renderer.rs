extern crate rayon;
extern crate rand;
extern crate indicatif;

use renderer::rayon::prelude::*;
use std::f32;
use std::time::{Instant};
use self::indicatif::{ProgressBar, ProgressStyle, HumanDuration};

use ::vec3::*;
use ray::Ray;
use geometry::*;
use camera::Camera;
use scene::Scene;

pub fn render(scene: &Scene, camera: &Camera, nx: usize, ny: usize, ns: usize) -> Vec<u8> {
  println!("{}", scene.bvh);

  let bar = &Box::new(ProgressBar::new((nx * ny / 64) as u64));
  bar.set_prefix("🎨  Rendering");
  bar.set_style(ProgressStyle::default_bar()
    .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%"));

  let start = Instant::now();

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

  bar.finish();

  println!("Finished in {}", HumanDuration(start.elapsed()));

  pixels
}

fn color(r: &Ray, scene: &Scene, depth: i32) -> Vec3 {
  let hit = scene.bvh.hit(&r, 0.001, f32::MAX);

  match hit {
    Some(rec) => {
      // println!("{:?} {:?}", depth, rec.p);
      let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
      if depth < scene.max_ray_depth {
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
