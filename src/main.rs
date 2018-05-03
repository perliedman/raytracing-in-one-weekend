extern crate rand;
extern crate png;

use std::f32;

mod vec3;
mod ray;
mod hitable;
mod bvh;
mod camera;
mod renderer;
mod aabb;
mod material;

use std::io;
use std::io::BufWriter;
use png::HasParameters;

use vec3::{Vec3, unit_vector};
use ray::Ray;
use hitable::*;
use material::*;
use camera::Camera;
use renderer::*;
use bvh::BvhTree;

fn main() {
  let nx = 320;
  let ny = 320;
  let ns = 500;

  // let pixels = render_random(nx, ny, ns);
  let pixels = render_cornell(nx, ny, ns);

  let ref mut w = BufWriter::new(io::stdout());

  let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
  encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
  let mut writer = encoder.write_header().unwrap();

  writer.write_image_data(&pixels).unwrap();
}

fn render_random(nx: usize, ny: usize, ns: usize) -> Vec<u8> {
  let lookfrom = Vec3::new(10.0, 1.8, 2.4);
  let lookat = Vec3::new(0.0, 0.0, 0.5);
  let dist_to_focus = (lookfrom-Vec3::new(4.0, 1.0, 0.0)).length();

  let camera = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    30.0,
    (nx as f32) / (ny as f32),
    0.1,
    dist_to_focus);

  let mut world = random_scene();
  let bvh = BvhTree::new(world.as_mut());
  let scene = Scene {
    model: &bvh,
    environment: &void
    // environment: &simple_sky
  };

  render(&scene, &camera, nx, ny, ns)
}

fn render_cornell(nx: usize, ny: usize, ns: usize) -> Vec<u8> {
  let lookfrom = Vec3::new(278.0, 278.0, -800.0);
  let lookat = Vec3::new(278.0, 278.0, 0.0);
  let dist_to_focus = 10.0;

  let camera = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    40.0,
    (nx as f32) / (ny as f32),
    0.0,
    dist_to_focus);

  let mut world = cornell_box();
  let bvh = BvhTree::new(world.as_mut());
  let scene = Scene {
    model: &bvh,
    environment: &void
  };

  render(&scene, &camera, nx, ny, ns)
}

fn random_scene() -> Vec<Box<Hitable>> {
  let scene_c = Vec3::new(4.0, 0.0, 2.0);

  let checker = CheckerTexture { odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)), even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)) };

  let mut models: Vec<Box<Hitable>> = vec![
    Box::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: Box::new(Lambertian { albedo: Box::new(checker) }) }),
    Box::new(Sphere { center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: Box::new(Dielectric { ref_idx: 1.5 }) }),
    Box::new(Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: Box::new(Lambertian { albedo: Box::new(ConstantTexture::new(0.4, 0.2, 0.1)) }) }),
    Box::new(Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: Box::new(Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 }) }),
    Box::new(XyRect { x0: -12.0, x1: -8.0, y0: 0.0, y1: 2.0, k: 2.0, material: Box::new(DiffuseLight { emit: Box::new(ConstantTexture::new(0.6*25.0, 0.55*25.0, 0.4*25.0)) }) })
  ];

  for a in -11..11 {
    for b in -11..11 {
      let center = Vec3::new((a as f32) + 0.9 * rand::random::<f32>(), 0.2, (b as f32) + 0.9 * rand::random::<f32>());

      if (center - scene_c).length() > 0.9 {
        let choose_mat = rand::random::<f32>();
        let material: Box<Material>;

        if choose_mat < 0.8 {
          material = Box::new(Lambertian {
            albedo: Box::new(ConstantTexture::new(
                          rand::random::<f32>() * rand::random::<f32>(),
                          rand::random::<f32>() * rand::random::<f32>(),
                          rand::random::<f32>() * rand::random::<f32>()))
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

        models.push(Box::new(Sphere { center, radius: 0.2, material }));
      }

    }
  }

  let world: Vec<Box<Hitable>> = models.into_iter().map(|s| s as Box<Hitable>).collect();
  world
}

fn cornell_box() -> Vec<Box<Hitable>> {
  let red = Lambertian { albedo: Box::new(ConstantTexture::new(0.65, 0.05, 0.05)) };
  let white1 = Lambertian { albedo: Box::new(ConstantTexture::new(0.73, 0.73, 0.73)) };
  let white2 = Lambertian { albedo: Box::new(ConstantTexture::new(0.73, 0.73, 0.73)) };
  let white3 = Lambertian { albedo: Box::new(ConstantTexture::new(0.73, 0.73, 0.73)) };
  let green = Lambertian { albedo: Box::new(ConstantTexture::new(0.12, 0.45, 0.15)) };
  let light = DiffuseLight { emit: Box::new(ConstantTexture::new(15.0, 15.0, 15.0)) };

  vec![
    Box::new(YzRect { y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 555.0, material: Box::new(green) }),
    Box::new(YzRect { y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 0.0, material: Box::new(red) }),
    Box::new(XzRect { x0: 213.0, x1: 343.0, z0: 227.0, z1: 332.0, k: 554.0, material: Box::new(light) }),
    Box::new(XzRect { x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 0.0, material: Box::new(white1) }),
    Box::new(XzRect { x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 555.0, material: Box::new(white2) }),
    Box::new(FlipNormals { hitable: Box::new(XyRect { x0: 0.0, x1: 555.0, y0: 0.0, y1: 555.0, k: 555.0, material: Box::new(white3) }) }),
  ]
}

fn simple_sky(r: &Ray) -> Vec3 {
  let unit_direction = unit_vector(r.direction);
  let t = 0.5 * (unit_direction.y() + 1.0);
  return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn void(_r: &Ray) -> Vec3 {
  Vec3::new(0.0, 0.0, 0.0)
}
