use std::ops;
use rand::{Rng, thread_rng};


#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3{
    e: [f32; 3],
}

impl Vec3{
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3{
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f32{
        self.e[0]
    }

    pub fn y(self) -> f32{
        self.e[1]
    }

    pub fn z(self) -> f32{
        self.e[2]
    }


    pub fn length(self) -> f32{
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32{
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn unit_vector(v: &Vec3) -> Vec3{
        *v / v.length()
    }

    pub fn write_color(self, samples_per_pixel: i32){
        let scale: f32 = 1.0 / (samples_per_pixel as f32);

        let r : f32 = (scale * self.e[0]).sqrt();
        let g : f32 = (scale * self.e[1]).sqrt();
        let b : f32 = (scale * self.e[2]).sqrt();

        let ir : u16 = (256.0 * Vec3::clamp(r, 0.0, 0.999)) as u16;
        let ig : u16 = (256.0 * Vec3::clamp(g, 0.0, 0.999)) as u16;
        let ib : u16 = (256.0 * Vec3::clamp(b, 0.0, 0.999)) as u16;

        println!("{} {} {}", ir, ig, ib);
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32{
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v:&Vec3) -> Vec3{
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]
        )
    }

    pub fn new_random_prob() -> f32 {
        let mut rng = thread_rng();
        rng.gen::<f32>()
    }

    pub fn new_random_vector (min: f32, max:f32) -> Vec3 {
        let mut rng = thread_rng();
        Vec3{
            e: [rng.gen::<f32>()*(max-min)+min, rng.gen::<f32>()*(max-min)+min, rng.gen::<f32>()*(max-min)+min]
        }
    }

    pub fn new_random_vector_in_unit_sphere() -> Vec3 {
        loop {
            let v : Vec3 = Vec3::new_random_vector(-1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }  
    }

    pub fn new_random_in_unit_disk() -> Vec3 {
        loop {
            let mut v: Vec3 = Vec3::new_random_vector(0.0, 1.0);
            v.e[2] = 0.0;
            if v.length_squared() < 1.0 {
                return v
            }
        }
    }

    pub fn new_random_unit_vector() -> Vec3{
        let mut rng = thread_rng();
        let a: f32 = rng.gen::<f32>() * 2.00 * 3.1415;  
        let z: f32 = rng.gen::<f32>() * 2.00 - 1.00;
        let r: f32 = (1.00 - z*z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * 2.0 * Vec3::dot(v, n)
    }

    pub fn refract(uv: &Vec3, n:&Vec3, etai_over_etat: f32) -> Vec3{
        let cos_theta: f32 = Vec3::dot(& -(*uv), n);
        let r_out_parallel: Vec3 = (*uv + (*n)*cos_theta) * etai_over_etat;
        let r_out_perp: Vec3 = (*n) * (1.0 - r_out_parallel.length_squared()).sqrt();
        //dbg!((*uv).length_squared(), etai_over_etat);
        return r_out_parallel - r_out_perp
    }

    // TODO: find appropriate place
    fn clamp(x: f32, min: f32, max: f32) -> f32{
        if x < min{
            return min;
        } else if x > max {
            return max;
        } else {
            return x;
        }
    }

}

    

impl ops::Add for Vec3{
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [self.e[0] + rhs.e[0], 
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]]
        }
    }
}

impl ops::AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Vec3){
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Mul for Vec3{
    type Output = Self;
    fn mul(self, rhs:Vec3) -> Self::Output{
        Vec3 { e: [self.e[0]* rhs.e[0],
            self.e[1]* rhs.e[1],
            self.e[2]* rhs.e[2]]
        }
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Self;
    fn mul(self, rhs:f32) -> Self::Output{
        Vec3 { e: [self.e[0]* rhs,
            self.e[1]* rhs,
            self.e[2]* rhs]
        }
    }
}

impl ops::Sub for Vec3{
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [self.e[0] - rhs.e[0], 
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]]
        }
    }
}


impl ops::Div<f32> for Vec3{
    type Output = Self;
    fn div(self, rhs:f32) -> Self::Output{
        Vec3 { e: [self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs]
        }
    }
}

impl ops::Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self::Output{
        Vec3 {e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}