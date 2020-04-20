use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;


#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian {
        albedo: Vec3
    },
    Metal {
        albedo: Vec3
    }
}


impl Default for Material{
    fn default() -> Self {
        Material::Lambertian{albedo: Vec3::new(0.0, 0.0, 0.0)}
    }
}


pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray
) -> bool {
    match material {
        &Material::Lambertian{albedo} => {
            let scatter_direction: Vec3 = rec.normal() + Vec3::new_random_unit_vector();
            *scattered = Ray::new(rec.p(), scatter_direction);
            *attenuation = albedo;
            return true;
        }
        &Material::Metal{albedo} => {
            let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(&ray_in.direction()), &rec.normal());
            *scattered = Ray::new(rec.p(), reflected);
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;
        }
    }
}


