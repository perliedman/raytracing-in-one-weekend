extern crate clap;
extern crate rand;
extern crate png;

use std::f32;
use std::fs::File;
use std::sync::Arc;
use clap::{App, Arg};

mod vec3;
mod mat44;
mod ray;
mod hitable;
mod bvh;
mod camera;
mod renderer;
mod aabb;
mod material;

use std::io::BufWriter;
use png::HasParameters;

use vec3::{Vec3, unit_vector};
use mat44::Mat44;
use ray::Ray;
use hitable::*;
use material::*;
use camera::Camera;
use renderer::*;
use bvh::BvhTree;

fn main() {
  let matches = App::new("plrt")
    .author("Per Liedman <per@liedman.net>")
    .about("Ray Tracer built from the book Ray Tracing in one weekend")
    .arg(Arg::with_name("output")
      .short("o")
      .long("output")
      .value_name("FILE")
      .help("image destination file")
      .takes_value(true))
    .arg(Arg::with_name("width")
      .short("w")
      .long("width")
      .value_name("WIDTH")
      .help("image width in pixels")
      .takes_value(true))
    .arg(Arg::with_name("height")
      .short("h")
      .long("height")
      .value_name("HEIGHT")
      .help("image height in pixels")
      .takes_value(true))
    .arg(Arg::with_name("samples")
      .short("s")
      .long("samples")
      .value_name("SAMPLES")
      .help("number of samples per pixel")
      .takes_value(true))
    .arg(Arg::with_name("max_ray_depth")
      .short("d")
      .long("max-ray-depth")
      .value_name("DEPTH")
      .help("maximum ray depth")
      .takes_value(true))
    .get_matches();

  let nx = matches.value_of("width").unwrap_or("320").parse::<usize>().unwrap();
  let ny = matches.value_of("height").unwrap_or("320").parse::<usize>().unwrap();
  let ns = matches.value_of("samples").unwrap_or("25").parse::<usize>().unwrap();;
  let max_ray_depth = matches.value_of("max_ray_depth").unwrap_or("10").parse::<i32>().unwrap();;

  // let pixels = render_random(nx, ny, ns);
  let pixels = render_cornell(nx, ny, ns, max_ray_depth);

  let path = matches.value_of("output").unwrap_or("a.png");
  let file = File::create(path).unwrap();
  let ref mut w = BufWriter::new(file);

  let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
  encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
  let mut writer = encoder.write_header().unwrap();

  writer.write_image_data(&pixels).unwrap();
}

fn render_random(nx: usize, ny: usize, ns: usize, max_ray_depth: i32) -> Vec<u8> {
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
    environment: Box::new(Void {}),
    max_ray_depth
  };

  render(&scene, &camera, nx, ny, ns)
}

fn render_cornell(nx: usize, ny: usize, ns: usize, max_ray_depth: i32) -> Vec<u8> {
  let lookfrom = Vec3::new(278.0, 278.0, -800.0);
  let lookat = Vec3::new(278.0, 278.0, 0.0);
  let dist_to_focus = 10.0;

  let camera = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    38.0,
    (nx as f32) / (ny as f32),
    0.0,
    dist_to_focus);

  let mut world = cornell_box();
  let bvh = BvhTree::new(world.as_mut());
  // eprintln!("{:?}", bvh);
  let scene = Scene {
    model: &bvh,
    environment: Box::new(Void {}),
    max_ray_depth
  };

  render(&scene, &camera, nx, ny, ns)
}

fn random_scene() -> Vec<Box<Hitable>> {
  let scene_c = Vec3::new(4.0, 0.0, 2.0);

  let checker = CheckerTexture { odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)), even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)) };

  let mut models: Vec<Box<Hitable>> = vec![
    Box::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: Arc::new(Lambertian { albedo: Box::new(checker) }) }),
    Box::new(Sphere { center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: Arc::new(Dielectric { ref_idx: 1.5 }) }),
    Box::new(Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: Arc::new(Lambertian { albedo: Box::new(ConstantTexture::new(0.4, 0.2, 0.1)) }) }),
    Box::new(Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: Arc::new(Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 }) }),
    Box::new(XyRect { x0: -12.0, x1: -8.0, y0: 0.0, y1: 2.0, k: 2.0, material: Arc::new(DiffuseLight { emit: Box::new(ConstantTexture::new(0.6*25.0, 0.55*25.0, 0.4*25.0)) }) })
  ];

  for a in -11..11 {
    for b in -11..11 {
      let center = Vec3::new((a as f32) + 0.9 * rand::random::<f32>(), 0.2, (b as f32) + 0.9 * rand::random::<f32>());

      if (center - scene_c).length() > 0.9 {
        let choose_mat = rand::random::<f32>();
        let material: Arc<Material>;

        if choose_mat < 0.8 {
          material = Arc::new(Lambertian {
            albedo: Box::new(ConstantTexture::new(
                          rand::random::<f32>() * rand::random::<f32>(),
                          rand::random::<f32>() * rand::random::<f32>(),
                          rand::random::<f32>() * rand::random::<f32>()))
          });
        } else if choose_mat < 0.95 {
          material = Arc::new(Metal {
            albedo: Vec3::new(
              0.5 * (1.0 + rand::random::<f32>()),
              0.5 * (1.0 + rand::random::<f32>()),
              0.5 * (1.0 + rand::random::<f32>())),
            fuzz: 0.5 * rand::random::<f32>(),
          })
        } else {
          material = Arc::new(Dielectric { ref_idx: 1.5 })
        }

        models.push(Box::new(Sphere { center, radius: 0.2, material }));
      }

    }
  }

  let world: Vec<Box<Hitable>> = models.into_iter().map(|s| s as Box<Hitable>).collect();
  world
}

fn cornell_box() -> Vec<Box<Hitable>> {
  let red = Arc::new(Lambertian { albedo: Box::new(ConstantTexture::new(0.65, 0.05, 0.05)) });
  let white: Arc<Material> = Arc::new(Lambertian { albedo: Box::new(ConstantTexture::new(0.73, 0.73, 0.73)) });
  let green = Arc::new(Lambertian { albedo: Box::new(ConstantTexture::new(0.12, 0.45, 0.15)) });
  let light = Arc::new(DiffuseLight { emit: Box::new(ConstantTexture::new(15.0, 15.0, 15.0)) });
  let dielectric: Arc<Material> = Arc::new(Dielectric { ref_idx: 1.8 });
  let subsurface: Vec<Box<Hitable>> = vec![
    Box::new(Sphere {center: Vec3::new(0.0, 0.0, 0.0), radius: 60.0, material: Arc::clone(&dielectric) }),
    Box::new(ConstantMedium::new(
      Box::new(Sphere {center: Vec3::new(0.0, 0.0, 0.0), radius: 60.0, material: Arc::clone(&white) }),
      0.2,
      Box::new(ConstantTexture::new(0.2, 0.4, 0.9))))
  ];

  vec![
    Box::new(FlipNormals { hitable: Box::new(YzRect { y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 555.0, material: green }) }),
    Box::new(YzRect { y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 0.0, material: red }),
    Box::new(XzRect { x0: 213.0, x1: 343.0, z0: 227.0, z1: 332.0, k: 554.0, material: light }),
    Box::new(XzRect { x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 0.0, material: Arc::clone(&white) }),
    Box::new(FlipNormals { hitable: Box::new(XzRect { x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 555.0, material: Arc::clone(&white) }) }),
    Box::new(FlipNormals { hitable: Box::new(XyRect { x0: 0.0, x1: 555.0, y0: 0.0, y1: 555.0, k: 555.0, material: Arc::clone(&white) }) }),
    Box::new(Transform::new(
      Box::new(new_box(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), Arc::clone(&white))),
      Mat44::translate(Vec3::new(130.0, 0.0, 65.0)) * Mat44::rotate(-18.0, Vec3::new(0.0, 1.0, 0.0))
    )),
    Box::new(Transform::new(
      Box::new(new_box(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), Arc::clone(&white))),
      Mat44::translate(Vec3::new(265.0, 0.0, 295.0)) * Mat44::rotate(15.0, Vec3::new(0.0, 1.0, 0.0))
    )),
    Box::new(Transform::new(
      Box::new(subsurface),
      Mat44::translate(Vec3::new(130.0, 225.0, 65.0)) * Mat44::rotate(-18.0, Vec3::new(0.0, 1.0, 0.0)) *
      Mat44::translate(Vec3::new(82.5, 0.0, 82.5))
    )),
  ]
}
