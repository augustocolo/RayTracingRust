mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;
extern crate rand;

use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, HitRecord};
use hittable_list::*;
use sphere::Sphere;
use camera::Camera;
use rand::{Rng, thread_rng};
use material::{Material, scatter};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vec3{
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, std::f32::MAX, &mut rec){
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Vec3 = Vec3::default();
        if scatter(&rec.material(), &r, &mut rec, &mut attenuation, &mut scattered){
            //dbg!(rec.material());
            return attenuation * ray_color(&scattered, &world, depth-1);
        }
        return Vec3::new(0.0, 0.0, 0.0)
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
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
    //eprintln!("{:?}", ss);
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian{albedo: Vec3::new(0.7, 0.3, 0.3)})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)})));
    list.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.6, 0.2)})));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.8, 0.8)})));

    let world = HittableList::new(list);

    // Construct a PPM file from image data
    let w: i32 = 2000;
    let h: i32 = 1000;
    let max_value: i32 = 255;
    let num_samples_per_pixel: i32 = 500;
    let max_depth: i32 = 5;
    let mut rng = thread_rng();

    println!("P3\n{} {}\n{}", w, h, max_value);
    for j in (0..h).rev(){
        eprintln!("Lines remaining: {}", j);
        for i in 0..w {
            let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..num_samples_per_pixel{
                let u: f32 = (i as f32 + rng.gen::<f32>()) / w as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>())/ h as f32;
                let r: Ray = camera.get_ray(u, v);
                color += ray_color(&r, &world, max_depth);
            }
            color.write_color(num_samples_per_pixel);
        }
    }

    eprintln!("Done!");
}
