use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
    front_face: bool,
    material: Material,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -(*outward_normal);
        }
    }

    pub fn t(self) -> f32{
        self.t
    }

    pub fn p(self) -> Vec3{
        self.p
    }

    pub fn normal(self) -> Vec3{
        self.normal
    }

    pub fn material(self) -> Material{
        self.material
    }

    pub fn set_t(&mut self, val:f32){
        self.t = val;
    }

    pub fn set_p(&mut self, val:Vec3){
        self.p = val;
    }

    pub fn set_normal(&mut self, val:Vec3){
        self.normal = val;
    }

    pub fn set_material(&mut self, val:Material){
        self.material = val;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord,) -> bool{
        false
    }
}