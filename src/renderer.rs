extern crate rand;

use std::f32;

use ::vec3::{Vec3, unit_vector};
use ray::Ray;
use hitable::*;
use camera::Camera;

pub fn render(world: &Hitable, camera: &Camera, nx: usize, ny: usize, ns: usize) -> Vec<u8> {
  let mut pixels: Vec<u8> = Vec::with_capacity(nx * ny * 3);

  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::new(0.0, 0.0, 0.0);

      for _s in 0..ns {
        let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
        let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);

        let r = camera.get_ray(u, v);
        col += color(r, *&world, 0);
      }

      col /= ns as f32;
      col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

      pixels.push((255.99 * col[0]) as u8);
      pixels.push((255.99 * col[1]) as u8);
      pixels.push((255.99 * col[2]) as u8);
    }
  }

  pixels
}

fn color(r: Ray, world: &Hitable, depth: i32) -> Vec3 {
  let hit = world.hit(&r, 0.001, f32::MAX);

  match hit {
    Some(rec) => {
      if depth < 50 {
        match rec.material.scatter(&r, &rec) {
          Some(scatter) => {
            if let Some(bounce) = scatter.ray {
              return scatter.color * color(bounce, world, depth + 1)
            }
          },
          None => {}
        }
      }

      return Vec3::new(0.0, 0.0, 0.0);
    },
    None => {
      let unit_direction = unit_vector(r.direction);
      let t = 0.5 * (unit_direction.y() + 1.0);
      return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
  }
}
