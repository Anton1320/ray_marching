use std::ops::{Mul, Rem};
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Rem<Vector3> for Vector3{
    type Output = f32;
    fn rem(self, a:Vector3) -> f32 {
        self.x*a.x+self.y*a.y+self.z*a.z
    }
}

impl Mul<f32> for Vector3{
    type Output = Vector3;
    fn mul(self, a:f32) -> Vector3 {
        Vector3 {x:self.x*a, y:self.y*a, z:self.z*a,}
    }
}

impl Vector3{
    pub fn length(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }
    pub fn norm(&self) -> Vector3 {
        let l = 1./self.length();
        Vector3 {x:self.x*l, y:self.y*l, z:self.z*l}
    }
}