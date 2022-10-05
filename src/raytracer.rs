use lazy_static::lazy_static;
use std::fs;
use vector3d::Vector3d;

struct Ray {
  origin: Vector3d<f32>,
  direction: Vector3d<f32>,
}

impl Ray {
  fn show(&self) -> String {
    return String::from(format!(
      "origin: {} {} {} \n direction: {} {} {}",
      self.origin.x,
      self.origin.y,
      self.origin.z,
      self.direction.x,
      self.direction.y,
      self.direction.z,
    ));
  }
}

static WIDTH: f32 = 600.0;
static HEIGHT: f32 = 600.0;
static SPHERE_RADIUS: f32 = 4.0;

fn per_pixel(x: f32, y: f32) -> String {
  let sphere_position: Vector3d<f32> = Vector3d::new(5.0, 5.0, 10.0);

  let ray = Ray {
    direction: Vector3d::new(0.0, 0.0, 1.0),
    origin: Vector3d::new((x / WIDTH) * 10.0, (y / HEIGHT) * 10.0, 0.0),
  };
  let ray_origin_position = Vector3d::new(
    ray.origin.x - sphere_position.x,
    ray.origin.y - sphere_position.y,
    ray.origin.z - sphere_position.z,
  );
  let a = ray.direction.dot(ray.direction);
  let b = 2.0 * ray_origin_position.dot(ray.direction);
  let c = ray_origin_position.dot(ray_origin_position) - SPHERE_RADIUS.powf(2.0);

  let discriminant = b.powf(2.0) - 4.0 * a * c;

  if discriminant < 0.0 {
    return String::from(format!(
      "{} {} 160 ",
      ((y as f32 / HEIGHT) * 200.0).round(),
      ((y as f32 / HEIGHT) * 200.0).round()
    ));
  }

  let hit = [
    (-b + discriminant.sqrt()) / (2.0 * a),
    (-b - discriminant.sqrt()) / (2.0 * a),
  ];

  let t = hit[0];
  let pos = ray.direction * t + ray.origin;
  let diff = pos - sphere_position;
  let len = (diff.x.powf(2.0) + diff.y.powf(2.0) + diff.z.powf(2.0)).sqrt();
  let mut normal = diff / len;

  normal.x = if normal.x < 0.0 { -normal.x } else { normal.x };
  normal.y = if normal.y < 0.0 { -normal.y } else { normal.y };
  normal.z = if normal.x < 0.0 { -normal.z } else { normal.z };
  // if normal.x < 0.0 || normal.y < 0.0 || normal.z < 0.0 {
  //   t = hit[0];
  //   pos = ray.direction * t + ray.origin;
  //   diff = pos - sphere_position;
  //   len = (diff.x.powf(2.0) + diff.y.powf(2.0) + diff.z.powf(2.0)).sqrt();
  //   normal = diff / len;
  // }

  return String::from(format!(
    "{} {} {} ",
    (normal.x * 255.0).round(),
    (normal.y * 255.0).round(),
    (normal.y * 255.0).round()
  ));
}

pub fn raytrace() {
  let mut buffer = String::from(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT));

  for y in 0..HEIGHT as i32 {
    for x in 0..WIDTH as i32 {
      buffer += &per_pixel(x as f32, y as f32);
    }
  }

  fs::write("image.ppm", buffer).expect("Unable to generate image");
}

// P3
// # feep.ppm
// 4 4
// 15
//  0  0  0    0  0  0    0  0  0   15  0 15
//  0  0  0    0 15  7    0  0  0    0  0  0
//  0  0  0    0  0  0    0 15  7    0  0  0
// 15  0 15    0  0  0    0  0  0    0  0  0
