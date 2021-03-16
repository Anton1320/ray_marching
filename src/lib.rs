use std::ops::{Mul, Rem, Sub, Add};
extern crate piston_window;
use piston_window::*;
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

impl Rem<f32> for Vector3 {
    type Output = Vector3;
    fn rem(self, a:f32) -> Vector3 {
        Vector3::new(self.x%a, self.y%a, self.z%a)
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
    pub fn new(x:f32, y:f32, z:f32) -> Vector3 {
        Vector3{x:x, y:y, z:z}
    }
    pub fn length(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }
    pub fn norm(&self) -> Vector3 {
        let l = 1./self.length();
        Vector3 {x:self.x*l, y:self.y*l, z:self.z*l}
    }
    pub fn abs(&self) -> Vector3 {
        Vector3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    fn max(&self, a:f32) -> Vector3 {
        Vector3 {
            x: self.x.max(a),
            y: self.y.max(a),
            z: self.z.max(a),
        }
    }
    fn maxcomp(&self) -> f32 {
        self.x.max(self.y.max(self.z))
    }
}

pub trait Figure {
    fn get_distance(&self, point: Vector3) -> f32;
    fn get_normal(&self, point: Vector3) -> Vector3 {
        const EPS: f32 = 0.0001;
        let d = self.get_distance(point);
        let dx = self.get_distance(point + Vector3::new(EPS, 0., 0.));
        let dy = self.get_distance(point + Vector3::new(0., EPS, 0.));
        let dz = self.get_distance(point + Vector3::new(0., 0., EPS));
        ((Vector3::new(dx, dy, dz) - Vector3::new(d, d, d)) * (1. / EPS)).norm()
    }
}

pub struct Translator {
    
}

impl Translator {
    fn new(figure: &dyn Figure, rot: Vector3) -> Translator {
        Translator{}
    }
}

pub struct Sphere {
    pub center: Vector3,
    pub r: f32,
    pub color: Vector3,
}

impl Figure for Sphere {
    fn get_distance(&self, point:Vector3) -> f32 {
        (point-self.center).length() - self.r
    }
}

pub struct Box {
    pub pos:Vector3,
    pub size: Vector3,
    pub rotation:Vector3,
    pub color: Vector3,
}

impl Box {
    pub fn new(pos:Vector3, rot: Vector3, size:Vector3, color:Vector3) -> Box {
        Box {
            pos:pos,
            rotation:rot,
            size: size,
            color: color,
        }
    }
    fn rotate_point(&self, point:Vector3, a:f32) -> Vector3 {
        let mut p = point;
        let mut p1 = point;
        let angle = self.rotation*a;
        p.z = p1.z*angle.x.cos()-p1.y*angle.x.sin();
        p.y = p1.z*angle.x.sin()+p1.y*angle.x.cos();
        p1 = p;
        //вокруг y
        p.x = p1.x*angle.y.cos()-p1.z*angle.y.sin();
        p.z = p1.x*angle.y.sin()+p1.z*angle.y.cos();
        p1 = p;
        //вокруг z
        p.x = p1.x*angle.z.cos()-p1.y*angle.z.sin();
        p.y = p1.x*angle.z.sin()+p1.y*angle.z.cos();
        p
    }
}
impl Figure for Box {
    fn get_distance(&self, point:Vector3) -> f32 {
        let p = self.rotate_point(point - self.pos, -1.);

        let q = p.abs() - self.size;
        q.max(0.).length() + q.maxcomp().min(0.)
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
    move_vec: Vector3,
    speed:f32,
}

impl Camera {
    pub fn new(pos:Vector3, screen_size:Vector3, screen_resolution:(usize, usize), dist_to_screen: f32,
                vector_to_screen: Vector3, angle_vector_x: Vector3) -> Camera {
        Camera {
            pos:pos,
            screen_size:screen_size,
            screen_resolution: screen_resolution,
            dist_to_screen: dist_to_screen,
            vector_to_screen: vector_to_screen,
            angle_vector_x: angle_vector_x,
            move_vec: Vector3::new(0., 0., 0.),
            speed: 0.1,
        }

    }
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
    pub fn button_handler(&mut self, btn: &ButtonArgs) {
        if let Button::Keyboard(i) = btn.button  {    
            if let ButtonState::Press = btn.state {
                match i {
                    Key::W => self.move_vec.x = 1.,
                    Key::S => self.move_vec.x = -1.,
                    Key::D => self.move_vec.z = 1.,
                    Key::A => self.move_vec.z = -1.,
                    Key::LShift => self.move_vec.y = 1.,
                    Key::Space => self.move_vec.y = -1.,
                    _ => (),
                }
            }
            else {
                match i {
                    Key::W => self.move_vec.x = 0.,
                    Key::S => self.move_vec.x = 0.,
                    Key::D => self.move_vec.z = 0.,
                    Key::A => self.move_vec.z = 0.,
                    Key::LShift => self.move_vec.y = 0.,
                    Key::Space => self.move_vec.y = 0.,
                    _ => (),
                }
            }
        }
    }
    pub fn move_pos(&mut self) {
        self.pos = self.pos + self.move_vec*self.speed;
    }
}