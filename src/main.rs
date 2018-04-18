extern crate rand;

use std::f32;

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
        let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
        let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);

        let r = camera.get_ray(u, v);
        col += color(r, &world);
      }

      col /= ns as f32;
      col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

      let ir = (255.99 * col[0]) as i32;
      let ig = (255.99 * col[1]) as i32;
      let ib = (255.99 * col[2]) as i32;

      println!("{} {} {}", ir, ig, ib);
    }
  }
}

fn color(r: Ray, world: &Hitable) -> Vec3 {
  let hit = world.hit(&r, 0.001, f32::MAX);

  match hit {
    Some(rec) => {
      let target = rec.p + rec.normal + random_in_unit_sphere();
      return 0.5 * color(Ray::new(rec.p, target - rec.p), world);
    },
    None => {
      let unit_direction = vec3::unit_vector(r.direction);
      let t = 0.5 * (unit_direction.y() + 1.0);
      return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
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