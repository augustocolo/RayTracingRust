use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ref_index: f32 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let scatter_direction: Vec3 = rec.normal() + Vec3::new_random_unit_vector();
            *scattered = Ray::new(rec.p(), scatter_direction);
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let mut f = 1.0;
            if fuzz < f {
                f = fuzz;
            }
            let reflected: Vec3 =
                Vec3::reflect(&Vec3::unit_vector(&ray_in.direction()), &rec.normal());
            *scattered = Ray::new(
                rec.p(),
                reflected + Vec3::new_random_vector_in_unit_sphere() * f,
            );
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;
        }
        &Material::Dielectric { ref_index } => {
            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            let etai_over_etat: f32;
            if rec.front_face() {
                etai_over_etat = 1.0 / ref_index;
                
            } else {
                etai_over_etat = ref_index;
            }

            let unit_direction: Vec3 = Vec3::unit_vector(&ray_in.direction());
            let cos_theta = Vec3::dot(&(-unit_direction), &rec.normal()).min(1.0);
            let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

            if etai_over_etat * sin_theta > 1.0 {
                let reflected: Vec3 = Vec3::reflect(&unit_direction, &rec.normal());
                *scattered = Ray::new(rec.p(), reflected);
                return true;
            }

            let reflect_prob: f32 = schlick(cos_theta, etai_over_etat);
            if Vec3::new_random_prob() < reflect_prob{
                let reflected: Vec3 = Vec3::reflect(&unit_direction, &rec.normal());
                *scattered = Ray::new(rec.p(), reflected);
                return true;
            }

            let refracted: Vec3 = Vec3::refract(&unit_direction, &rec.normal(), etai_over_etat);
            *scattered = Ray::new(rec.p(), refracted);
            true
        }
    }
}

fn schlick(cos: f32, ri: f32) -> f32 {
    let mut r0: f32 = (1.0 - ri) / (1.0 + ri);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
