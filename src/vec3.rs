use std::ops;


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

        let r : f32 = scale * self.e[0];
        let g : f32 = scale * self.e[1];
        let b : f32 = scale * self.e[2];

        let ir : u16 = (256.0 * Vec3::clamp(r, 0.0, 0.999)) as u16;
        let ig : u16 = (256.0 * Vec3::clamp(g, 0.0, 0.999)) as u16;
        let ib : u16 = (256.0 * Vec3::clamp(b, 0.0, 0.999)) as u16;

        println!("{} {} {}", ir, ig, ib);
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32{
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(2.0, 5.0, 3.25) + Vec3::new(5.5, 2.2, 3.2), 
            Vec3::new(7.5, 7.2, 6.45)
        )
    }

    #[test]
    fn test_vec3_mul() {
        assert_eq!(
            Vec3::new(2.0, 5.0, 3.5) * Vec3::new(5.5, 2.2, 4.0), 
            Vec3::new(11.0, 11.0, 14.0)
        )
    }

    #[test]
    fn test_vec3_mul_scalar() {
        assert_eq!(
            Vec3::new(2.0, 5.0, 3.25) * 2.0, 
            Vec3::new(4.0, 10.0, 6.5)
        )
    }

    #[test]
    fn test_vec3_div_scalar(){
        assert_eq!(
            Vec3::new(2.0, 5.0, 10.0) / 2.0, 
            Vec3::new(1.0, 2.5, 5.0)
        )
    }

    #[test]
    fn test_vec3_length(){
        unimplemented!();
    }

    #[test]
    fn test_vec3_length_squared(){
        unimplemented!();
    }
}