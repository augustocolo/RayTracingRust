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

fn random_scene() -> HittableList{

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::new(Vec3::new(0.0,-1000.0, 0.0), 1000.0, Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)})));

    for a in -11..11{
        for b in -11..11{
            let choose_mat: f32 = Vec3::new_random_prob();
            
            let center: Vec3 = Vec3::new(a as f32 + 0.9 * Vec3::new_random_prob(), 
            0.2, 
            b as f32 + 0.9*Vec3::new_random_prob());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Lambertian
                    let albedo: Vec3 = Vec3::new_random_vector(0.0, 1.0) * Vec3::new_random_vector(0.0, 1.0);
                    list.push(Box::new(Sphere::new(center , 0.2, Material::Lambertian{albedo: albedo})));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Vec3 = Vec3::new_random_vector(0.5, 1.0);
                    let fuzz: f32 = Vec3:: new_random_vector(0.0, 0.5).x();
                    list.push(Box::new(Sphere::new(center , 0.2, Material::Metal{albedo: albedo, fuzz:fuzz})));
                } else {
                    // Dielectric
                    list.push(Box::new(Sphere::new(center , 0.2, Material::Dielectric{ref_index: 1.5})));
                }
            }
        }
    }
    // Big Dielectric
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0) , 1.0, Material::Dielectric{ref_index: 1.5})));
    // Big Lambertian
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0) , 1.0, Material::Lambertian{albedo: Vec3::new(0.4, 0.2, 0.1)})));
    // Big Metal
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0) , 1.0, Material::Metal{albedo: Vec3::new(0.7, 0.6, 0.5), fuzz:0.0})));
    
    return HittableList::new(list);
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vec3{
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, std::f32::MAX, &mut rec){
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Vec3 = Vec3::default();
        if scatter(&rec.material(), &r, &mut rec, &mut attenuation, &mut scattered){
            //dbg!(attenuation, ray_color(&scattered, &world, depth-1));
            return attenuation * ray_color(&scattered, &world, depth-1);
        }
        return Vec3::new(0.0, 0.0, 0.0)
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut rng = thread_rng();

    // Setting the world
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    //eprintln!("{:?}", ss);
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian{albedo: Vec3::new(0.1, 0.2, 0.5)})));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)})));
    list.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 0.3})));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric{ref_index: 1.5})));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Material::Dielectric{ref_index: 1.5})));


    let world = random_scene();

    // Construct a PPM file from image data
    let w: i32 = 1920;
    let h: i32 = 1080;
    let max_value: i32 = 255;
    let num_samples_per_pixel: i32 = 250;
    let max_depth: i32 = 10;
    // Setting the camera
    let aspect_ratio: f32 = w as f32 / h as f32;
    let look_from: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let look_at: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let world_vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.1;
    let camera: Camera = Camera::new(look_from, look_at, world_vup, 20.0, aspect_ratio, aperture, dist_to_focus);

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
