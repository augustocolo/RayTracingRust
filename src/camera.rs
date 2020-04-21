use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3, 
        look_at: Vec3, 
        v_up: Vec3, 
        vfov: f32, 
        aspect: f32, 
        aperture: f32,
        focus_dist: f32    
    ) -> Camera {

        let origin = look_from;
        let lens_radius = aperture / 2.0;
        

        let theta: f32 = Camera::degrees_to_radians(vfov);
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect * half_height;
        let w: Vec3 = Vec3::unit_vector(&(look_from - look_at));
        let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        let v: Vec3 = Vec3::cross(&w, &u);
        

        let lower_left_corner: Vec3 = origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal: Vec3 = u * half_width * 2.00 * focus_dist;
        let vertical: Vec3 = v * half_height * 2.00 * focus_dist;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(self, i: f32, j: f32) -> Ray {
        let rd: Vec3 = Vec3::new_random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * i + self.vertical * j - self.origin - offset,
        )
    }

    fn degrees_to_radians(deg: f32) -> f32 {
        deg * std::f32::consts::PI / 180.0
    }
}
