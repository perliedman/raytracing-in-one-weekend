mod vec3;

fn main() {
  let nx = 200;
  let ny = 100;

  println!("P3");
  println!("{} {}", nx, ny);
  println!("255");

  for j in (0..ny).rev() {
    for i in 0..nx {
      let r = (i as f64) / (nx as f64);
      let g = (j as f64) / (ny as f64);
      let b = 0.2;

      let ir = (255.99 * r) as i32;
      let ig = (255.99 * g) as i32;
      let ib = (255.99 * b) as i32;

      println!("{} {} {}", ir, ig, ib);
    }
  }
}
