mod vec3;

fn main() {
  let nx = 200;
  let ny = 100;

  println!("P3");
  println!("{} {}", nx, ny);
  println!("255");

  for j in (0..ny).rev() {
    for i in 0..nx {
      let col = vec3::build_vec3((i as f32) / (nx as f32), (j as f32) / (ny as f32), 0.2);

      let ir = (255.99 * col[0]) as i32;
      let ig = (255.99 * col[1]) as i32;
      let ib = (255.99 * col[2]) as i32;

      println!("{} {} {}", ir, ig, ib);
    }
  }
}
