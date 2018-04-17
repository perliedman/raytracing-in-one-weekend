use std::f32;

mod vec3;
mod ray;
mod hitable;

use vec3::Vec3;
use ray::Ray;
use hitable::*;

fn main() {
  let nx = 200;
  let ny = 100;

  println!("P3");
  println!("{} {}", nx, ny);
  println!("255");

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  let spheres = vec![
    Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 },
    Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 },
  ];
  let world: Vec<Box<Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<Hitable>).collect();

  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = (i as f32) / (nx as f32);
      let v = (j as f32) / (ny as f32);

      let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
      let col = color(r, &world);

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
    Some(hitRecord) => {
      let n = hitRecord.normal;
      return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    },
    None => {
      let unit_direction = vec3::unit_vector(r.direction);
      let t = 0.5 * (unit_direction.y() + 1.0);
      return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
  }
}
