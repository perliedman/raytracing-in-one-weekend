extern crate rand;

use std::f32;
use rand::{Rng, thread_rng};

mod vec3;
mod ray;
mod hitable;
mod camera;

use vec3::Vec3;
use ray::Ray;
use hitable::*;
use camera::Camera;

fn main() {
  let nx = 200;
  let ny = 100;
  let ns = 100;

  let mut rng = rand::thread_rng();

  println!("P3");
  println!("{} {}", nx, ny);
  println!("255");

  let camera = Camera {
    lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
    horizontal: Vec3::new(4.0, 0.0, 0.0),
    vertical: Vec3::new(0.0, 2.0, 0.0),
    origin: Vec3::new(0.0, 0.0, 0.0),
  };

  let spheres = vec![
    Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 },
    Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 },
  ];
  let world: Vec<Box<Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<Hitable>).collect();

  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::new(0.0, 0.0, 0.0);

      for _s in 0..ns {
        let u = ((i as f32) + rng.gen::<f32>()) / (nx as f32);
        let v = ((j as f32) + rng.gen::<f32>()) / (ny as f32);

        let r = camera.get_ray(u, v);
        col += color(r, &world);
      }

      col /= ns as f32;

      let ir = (255.99 * col[0]) as i32;
      let ig = (255.99 * col[1]) as i32;
      let ib = (255.99 * col[2]) as i32;

      println!("{} {} {}", ir, ig, ib);
    }
  }
}

fn color(r: Ray, world: &Hitable) -> Vec3 {
  let hit = world.hit(&r, 0.0, f32::MAX);

  match hit {
    Some(hit_record) => {
      let n = hit_record.normal;
      return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    },
    None => {
      let unit_direction = vec3::unit_vector(r.direction);
      let t = 0.5 * (unit_direction.y() + 1.0);
      return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
  }
}
