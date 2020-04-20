mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
extern crate rand;

use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, HitRecord};
use hittable_list::*;
use sphere::Sphere;
use camera::Camera;
use rand::{Rng, thread_rng};

fn ray_color(r: &Ray, world: &HittableList) -> Vec3{
    let mut rec = HitRecord::default();

    if world.hit(&r, 0.0, std::f32::MAX, &mut rec){
        let v: Vec3 = Vec3::new(
            rec.normal().x() + 1.0,
            rec.normal().y() + 1.0,
            rec.normal().z() + 1.0
        ) * 0.5;
        return v;
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32{
    let oc: Vec3 = r.origin() - *center;
    let a : f32= Vec3::dot(&r.direction(), &r.direction());
    let half_b: f32 = Vec3::dot(&oc, &r.direction());
    let c : f32= Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant: f32= half_b*half_b - a*c;

    if discriminant < 0.0{
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}


fn main() {
    

    // Setting the camera
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera: Camera = Camera::new(lower_left_corner, horizontal, vertical, origin);


    // Setting the world
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    // Construct a PPM file from image data
    let w: i32 = 2000;
    let h: i32 = 1000;
    let max_value: i32 = 255;
    let num_samples_per_pixel: i32 = 10;
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n{}", w, h, max_value);
    for j in (0..h).rev(){
        eprintln!("Lines remaining: {}", j);
        for i in 0..w {
            let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..num_samples_per_pixel{
                let u: f32 = (i as f32 + rng.gen::<f32>()) / w as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>())/ h as f32;
                let r: Ray = camera.get_ray(u, v);
                color += ray_color(&r, &world);
            }
            color.write_color(num_samples_per_pixel);
        }
    }

    eprintln!("Done!");
}
