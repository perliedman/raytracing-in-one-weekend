use std::f32;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

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

  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = (i as f32) / (nx as f32);
      let v = (j as f32) / (ny as f32);

      let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
      let col = color(r);

      let ir = (255.99 * col[0]) as i32;
      let ig = (255.99 * col[1]) as i32;
      let ib = (255.99 * col[2]) as i32;

      // if ir < 0 {
      //   println!("{} {} {}", col[0], col[1], col[2]);
      // }

      println!("{} {} {}", ir, ig, ib);
    }
  }
}

fn color(r: Ray) -> Vec3 {
  let t = hit_sphere(&Vec3::new(0., 0., -1.), 0.5, &r);

  if t >= 0.0 {
    let n = vec3::unit_vector(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
    return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
  }

  let unit_direction = vec3::unit_vector(r.direction);
  let t = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
  let oc = r.origin - *center;
  let a = r.direction.dot(r.direction);
  let b = 2. * oc.dot(r.direction);
  let c = oc.dot(oc) - 2. * radius * radius;
  let discriminant = b * b - 4. * a * c;

  if discriminant < 0. {
    return -1.0;
  } else {
    return (-b - discriminant.sqrt()) / (2.0 * a);
  }
}
