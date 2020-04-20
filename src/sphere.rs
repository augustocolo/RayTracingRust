use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::*;
use crate::material::Material;

#[derive(Debug)]
pub struct Sphere{
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere{
        Sphere {
            center, radius, material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = r.origin() - self.center;
        let a : f32=  r.direction().length_squared();
        let half_b: f32 = Vec3::dot(&oc, &r.direction());
        let c : f32= oc.length_squared() - self.radius*self.radius;
        let discriminant: f32= half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root: f32 = discriminant.sqrt();
            let mut temp: f32 = (- half_b - root) / a;
            if temp < t_max && temp > t_min{
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                let outward_normal: Vec3= (rec.p() - self.center) / self.radius; 
                rec.set_face_normal(r, &outward_normal);
                rec.set_material(self.material);
                return true;
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                let outward_normal: Vec3= (rec.p() - self.center) / self.radius; 
                rec.set_face_normal(r, &outward_normal);
                rec.set_material(self.material);
                return true;
            }
        }
        false
    }
}