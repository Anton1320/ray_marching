use std::ops::{Mul, Rem, Sub, Add};
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Rem<Vector3> for Vector3{
    type Output = Vector3;
    fn rem(self, a:Vector3) -> Vector3 {
        Vector3 {
            x:self.y*a.z-self.z*a.y,
            y:self.z*a.x-self.x*a.z,
            z:self.x*a.y-self.y*a.x,
        }
    }
}

impl Mul<f32> for Vector3{
    type Output = Vector3;
    fn mul(self, a:f32) -> Vector3 {
        Vector3 {x:self.x*a, y:self.y*a, z:self.z*a,}
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = f32;
    fn mul(self, a:Vector3) -> f32 {
        self.x*a.x+self.y*a.y+self.z*a.z
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, a:Vector3) -> Vector3 {
        Vector3 {
            x:self.x-a.x,
            y:self.y-a.y,
            z:self.z-a.z,
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, a:Vector3) -> Vector3 {
        Vector3 {
            x:self.x+a.x,
            y:self.y+a.y,
            z:self.z+a.z,
        }
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

pub struct Sphere {
    center: Vector3,
    r: f32,
}

impl Sphere {
    pub fn normal(&self, point:Vector3) -> Vector3 {
        
        (point-self.center).norm()
    }
    pub fn get_distance(&self, point:Vector3) -> f32 {
        (point-self.center).length() - self.r
    }
}

pub struct Screen {
    pub pos:Vector3, // координата левого верхнего угла
    pub i: Vector3, // еденичный вектор, параллельный стороне x и выходящий из левого нижнего угла
    pub j: Vector3, // еденичный вектор, параллельный стороне y и выходящий из левого нижнего угла
}

pub struct Camera {
    pub pos: Vector3, // координата экрана
    pub screen_size: Vector3, // Vector 2 (x, y) размер экрана
    pub screen_resolution: (usize, usize), // разрешение экрана (x, y) (width, height)
    pub dist_to_screen: f32, // расстояние от центра экрана до камеры
    pub vector_to_screen: Vector3, // нормированный вектор из камеры, указывающий на центр экрана
    pub angle_vector_x: Vector3, // нормированный вектор из центра экрана в середину правой стороны экрана
}

impl Camera {
    pub fn get_screen(&self) -> Screen {
        let v_x = self.angle_vector_x*self.screen_size.x;
        let v_y = (v_x%self.vector_to_screen).norm()*self.screen_size.y;
        let pos = self.pos+self.vector_to_screen*self.dist_to_screen-(v_x+v_y)*0.5;
        Screen {
            pos: pos,
            i: v_x.norm(),
            j: v_y.norm(),
        }
    }

    pub fn get_render_vectors(&self) -> Vec<Vec<Vector3>>{
        let sc = self.get_screen();
        let mut out: Vec<Vec<Vector3>> = vec![];
        for i in 0..self.screen_resolution.0 {
            out.push(vec![]);
            for j in 0..self.screen_resolution.1 {
                out[i].push((sc.pos+sc.i*(self.screen_size.x/self.screen_resolution.0 as f32)*(i as f32)+
                                   sc.j*(self.screen_size.y/self.screen_resolution.1 as f32)*(j as f32)-self.pos).norm());
            }
        }
        out
    }
}