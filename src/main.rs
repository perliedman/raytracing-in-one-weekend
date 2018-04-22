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
  let nx = 320;
  let ny = 320;
  let ns = 150;

  let lookfrom = Vec3::new(10.0, 1.8, 2.4);
  let lookat = Vec3::new(0.0, 0.0, 0.5);
  // let dist_to_focus = (lookfrom-lookat).length();
  let dist_to_focus = (lookfrom-Vec3::new(4.0, 1.0, 0.0)).length();

  let camera = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    30.0,
    (nx as f32) / (ny as f32),
    0.1,
    dist_to_focus);

  let world = random_scene();

  println!("P3");
  println!("{} {}", nx, ny);
  println!("255");
  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::new(0.0, 0.0, 0.0);

      for _s in 0..ns {
        let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
        let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);

        let r = camera.get_ray(u, v);
        col += color(r, &world, 0);
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
      let unit_direction = vec3::unit_vector(r.direction);
      let t = 0.5 * (unit_direction.y() + 1.0);
      return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
  }
}

fn random_scene() -> Vec<Box<Hitable>> {
  let scene_c = Vec3::new(4.0, 0.0, 2.0);

  let mut spheres = vec![
    Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: Box::new(Lambertian { albedo: Vec3::new(0.7, 0.26, 0.10) }) },
    Sphere { center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: Box::new(Dielectric { ref_idx: 1.5 }) },
    Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: Box::new(Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) }) },
    Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: Box::new(Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 }) }
  ];

  for a in -11..11 {
    for b in -11..11 {
      let center = Vec3::new((a as f32) + 0.9 * rand::random::<f32>(), 0.2, (b as f32) + 0.9 * rand::random::<f32>());

      if (center - scene_c).length() > 0.9 {
        let choose_mat = rand::random::<f32>();
        let material: Box<Material>;

        if choose_mat < 0.8 {
          material = Box::new(Lambertian {
            albedo: Vec3::new(
              rand::random::<f32>() * rand::random::<f32>(),
              rand::random::<f32>() * rand::random::<f32>(),
              rand::random::<f32>() * rand::random::<f32>())
          });
        } else if choose_mat < 0.95 {
          material = Box::new(Metal {
            albedo: Vec3::new(
              0.5 * (1.0 + rand::random::<f32>()),
              0.5 * (1.0 + rand::random::<f32>()),
              0.5 * (1.0 + rand::random::<f32>())),
            fuzz: 0.5 * rand::random::<f32>(),
          })
        } else {
          material = Box::new(Dielectric { ref_idx: 1.5 })
        }

        spheres.push(Sphere { center, radius: 0.2, material });
      }

    }
  }

  let world: Vec<Box<Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<Hitable>).collect();
  world
}