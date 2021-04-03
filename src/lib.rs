use std::ops::{Mul, Rem, Sub, Add, IndexMut};
extern crate piston_window;
extern crate image;
use piston_window::*;
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct Matrix4x4 {
    m: [[f32; 4]; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub position: Matrix4x4,
    pub rotation: Matrix4x4,
    pub size: Matrix4x4,
    pub matrix: Matrix4x4,
}

impl Transform {
    pub fn new(pos: Vector3, rot: Vector3, size: Vector3) -> Transform {
        let a = Matrix4x4::new_pos_matrix(pos);
        let b = Matrix4x4::new_rotation_matrix(rot, None);
        let c = Matrix4x4::new_size_matrix(size);
        Transform {
            position: a,
            rotation: b,
            size: c,
            matrix: b*c*a, 
        }
    }
    pub fn transform_matrix(&mut self, pos: Vector3, rot: Vector3, size:Vector3) {
        self.position.m[0][3] -= pos.x;
        self.position.m[1][3] += pos.y;
        self.position.m[2][3] += pos.z;
        self.size.m[0][0] += size.x;
        self.size.m[1][1] += size.y;
        self.size.m[2][2] += size.z;
        self.rotation = self.rotation * Matrix4x4::new_rotation_matrix(rot, None);
        self.matrix = self.size * self.rotation * self.position;
    }
}

impl Matrix4x4 {
    fn new_empty_matrix() -> Matrix4x4 {

        Matrix4x4 {m: [[0.,0.,0.,0.,],[0.,0.,0.,0.,],[0.,0.,0.,0.,],[0.,0.,0.,0.,]]}
    }
    fn new_pos_matrix(pos:Vector3) -> Matrix4x4 {
        Matrix4x4{m:
            [[1., 0., 0., -pos.x],
             [0., 1., 0., pos.y],
             [0., 0., 1., pos.z],
             [0., 0., 0.,   1.]]
        }
    }
    fn new_rotation_matrix(rot: Vector3, freeze: Option<(bool, bool, bool)>) -> Matrix4x4 {
        let set_rot_x = Matrix4x4{m:
            [[1., 0.,          0.,           0.],
             [0., rot.x.cos(), -rot.x.sin(), 0.],
             [0., rot.x.sin(), rot.x.cos(),  0.],
             [0., 0.,          0.,           1.]]
        };
        let set_rot_y = Matrix4x4{m:
            [[(rot.y).cos(), 0., (rot.y).sin(), 0.],
             [0.,             1., 0.,              0.],
             [-(rot.y).sin(), 0., (rot.y).cos(),  0.],
             [0.,             0., 0.,              1.]]
        };
        let set_rot_z = Matrix4x4{m:
            [[rot.z.cos(), -rot.z.sin(), 0., 0.],
             [rot.z.sin(), rot.z.cos(),  0., 0.],
             [0.,          0.,           1., 0.],
             [0.,          0.,           0., 1.]]
        };
        let mut out = Matrix4x4::new_pos_matrix(Vector3::new(0., 0., 0.));
        if let Some(i) = freeze{
            if !i.0{ out = out * set_rot_x; }
            if !i.1{ out = out * set_rot_y; }
            if !i.2{ out = out * set_rot_z; }
        }
        else {
            out = set_rot_y*out*set_rot_z*set_rot_x;
        }
        out
    }
    pub fn new_size_matrix(size: Vector3) -> Matrix4x4 {
        Matrix4x4 {
            m:[[size.x, 0., 0., 0.],
             [0., size.y, 0., 0.],
             [0., 0., size.z, 0.],
             [0., 0., 0., 0.]]
        }
    }
}

impl Mul<Vector3> for Matrix4x4 {
    type Output = Vector3;
    fn mul(self, a:Vector3) -> Vector3 {
        Vector3::new(
            self.m[0][0]*a.x+self.m[0][1]*a.y+self.m[0][2]*a.z+self.m[0][3], 
            self.m[1][0]*a.x+self.m[1][1]*a.y+self.m[1][2]*a.z+self.m[1][3],
            self.m[2][0]*a.x+self.m[2][1]*a.y+self.m[2][2]*a.z+self.m[2][3]
        )
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, a:Matrix4x4) -> Matrix4x4 {
        let mut b = Matrix4x4::new_empty_matrix();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    b.m[i][j] += self.m[i][k]*a.m[k][j];
                }
            }
        }
        b
    }
}
#[test]
fn test() {
    let a = Matrix4x4{m:[[ 12.,  23.,   7.,   6.],
        [ 34.,  85.,  12.,  65.],
        [594., 374., 895., 385.],
        [214.,  45.,  85.,  98.]]}*
        Matrix4x4 {
            m:[[0.49083904, 0.78827533, 0.65304331, 0.79109184],
       [0.48680807, 0.84175652, 0.02830482, 0.15441525],
       [0.54371187, 0.25212335, 0.74663694, 0.44069122],
       [0.30097199, 0.87539356, 0.94871428, 0.9823001 ]]
        };
    let b = Matrix4x4 {m:[[  22.69846904,   35.83692878,   19.40627489,   22.02329201],
        [  84.15493491,  158.27672724,   95.23545377,  109.1602201 ],
        [1076.12094689, 1345.72940173, 1431.98878752, 1300.26403771],
        [ 202.65668101,  313.78901793,  297.46312495,  309.96650304]]};
    let mut c = true;
    for i in 1..4 {
        for j in 1..4 {
            if (a.m[i][j] - b.m[i][j]).abs() > 0.0001{
                c = false;
                println!("{}, {}\n{}", i, j, a.m[i][j]);
                break;
            }
        }
    }
    assert_eq!(c, true)
}

impl Vector3{
    pub fn new(x:f32, y:f32, z:f32) -> Vector3 {
        Vector3{x:x, y:y, z:z}
    }
    pub fn length(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }
    pub fn norm(&self) -> Vector3 {
        let a = self.length();
        let l: f32;
        if a != 0. {l = 1./self.length();}
        else {l = 0.;}
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
    /*fn min(&self, a:f32) -> Vector3 {
        Vector3 {
            x: self.x.min(a),
            y: self.y.min(a),
            z: self.z.min(a),
        }
    }*/
    fn maxcomp(&self) -> f32 {
        self.x.max(self.y.max(self.z))
    }
}

pub trait Figure {
    fn get_distance(&self, point: Vector3) -> f32;
    fn get_transform(&self) -> &Transform;
    fn get_mut_transform(&mut self) -> &mut Transform;
    fn get_figure_color(&self) -> Vector3;
    fn get_normal(&self, point: Vector3) -> Vector3 {
        const EPS: f32 = 0.0001;
        let d = self.get_distance(point);
        let dx = self.get_distance(point + Vector3::new(EPS, 0., 0.));
        let dy = self.get_distance(point + Vector3::new(0., EPS, 0.));
        let dz = self.get_distance(point + Vector3::new(0., 0., EPS));
        ((Vector3::new(dx, dy, dz) - Vector3::new(d, d, d)) * (1. / EPS)).norm()
    }
    //fn get_transform(&self) -> &Transform;
    fn change_transform(&mut self, pos:Vector3, rot: Vector3, size: Vector3) {
        self.get_mut_transform().transform_matrix(pos, rot, size);
    }
    fn get_color(&self, point:Vector3, light: Vector3) -> Vector3 {
        let l = (self.get_normal(point)*(light-point).norm()+1.5).max(0.)/3.;
        self.get_figure_color()*l
    }
}

pub struct Folder<'a> {
    pub figures: Vec<&'a mut dyn Figure>,
    pub transform: Transform,
}

impl Folder<'_> {
    fn get_closere_object(&self, point: Vector3) -> (f32, usize) {
        let mut m = 1000000000.;
        let mut a = 0 as usize;
        let mut j = 0 as usize;
        for i in &self.figures {
            let d = i.get_distance(point);
            if d < m {
                m = d;
                a = j;
            }
            j += 1;
        }
        (m, a)
    }
}

impl Figure for Folder<'_> {
    fn get_distance(&self, point: Vector3) -> f32 {
        let p = self.transform.rotation * point;
        self.get_closere_object(p).0
    }
    fn get_transform(&self) -> &Transform { &self.transform }
    fn get_mut_transform(&mut self) -> &mut Transform { &mut self.transform }
    fn get_figure_color(&self) -> Vector3 { Vector3::new(0., 0., 0.) }
    fn get_color(&self, point: Vector3, light: Vector3) -> Vector3 {
        let p = self.transform.rotation * point;
        self.figures[self.get_closere_object(p).1].get_color(p, light)
    }
}

pub struct Plane {
    pub y: f32,
    pub transform: Transform,
    pub color: Vector3,
}

impl Figure for Plane {
    fn get_distance(&self, point: Vector3) -> f32 {
        self.y - point.y
    }
    fn get_transform(&self) -> &Transform { &self.transform }
    fn get_mut_transform(&mut self) -> &mut Transform { &mut self.transform }
    fn get_figure_color(&self) -> Vector3 {self.color}
}

pub struct Sphere {
    pub center: Vector3,
    pub r: f32,
    pub color: Vector3,
    pub transform: Transform,
}

impl Sphere {
    pub fn new(pos:Vector3, rot: Vector3, radius: f32, color:Vector3) -> Sphere {
        Sphere {
            center:pos,
            r: radius,
            color: color,
            transform: Transform::new(pos, rot,Vector3::new(1., 1., 1.)),
        }
    }
}

impl Figure for Sphere {
    fn get_distance(&self, point:Vector3) -> f32 {
        let p = self.transform.matrix * point;
        (p).length() - self.r
    }
    fn get_transform(&self) -> &Transform { &self.transform }
    fn get_mut_transform(&mut self) -> &mut Transform { &mut self.transform }
    fn get_figure_color(&self) -> Vector3 {self.color}
}


pub struct Box {
    pub color: Vector3,
    pub transform: Transform,
}

impl Box {
    pub fn new(pos:Vector3, rot: Vector3, size:Vector3, color:Vector3) -> Box {
        Box {
            color: color,
            transform: Transform::new(pos, rot, size),
        }
    }
}
impl Figure for Box {
    fn get_distance(&self, point:Vector3) -> f32 {
        let p =  self.transform.matrix*point;
        let q = p.abs() - Vector3::new(1., 1., 1.);
        let d = q.max(0.).length() + q.maxcomp().min(0.) - 0.2;
        d
    }
    fn get_transform(&self) -> &Transform { &self.transform }
    fn get_mut_transform(&mut self) -> &mut Transform { &mut self.transform }
    fn get_figure_color(&self) -> Vector3 { self.color }
}


struct Screen {
    pos:Vector3, // координата левого верхнего угла
    i: Vector3, // еденичный вектор, параллельный стороне x и выходящий из левого нижнего угла
    j: Vector3, // еденичный вектор, параллельный стороне y и выходящий из левого нижнего угла
}

pub struct Camera {
    pub pos: Vector3, // координата экрана
    pub screen_size: Vector3, // Vector 2 (x, y) размер экрана
    pub screen_resolution: (usize, usize), // разрешение экрана (x, y) (width, height)
    pub dist_to_screen: f32, // расстояние от центра экрана до камеры
    pub vector_to_screen: Vector3, // нормированный вектор из камеры, указывающий на центр экрана
    pub angle_vector_x: Vector3, // нормированный вектор из центра экрана в середину правой стороны экрана
    pub transform: Transform,
    move_vec: Vector3,
    rot_vec: Vector3,
    speed:f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new(pos:Vector3, screen_size:Vector3, screen_resolution:(usize, usize), dist_to_screen: f32) -> Camera {
        Camera {
            pos:pos,
            screen_size:screen_size,
            screen_resolution: screen_resolution,
            dist_to_screen: dist_to_screen,
            vector_to_screen: Vector3::new(1., 0., 0.),
            angle_vector_x: Vector3::new(0., 0., 1.),
            transform: Transform::new(pos, Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.)),
            move_vec: Vector3::new(0., 0., 0.),
            rot_vec: Vector3::new(0., 0., 0.),
            speed: 0.1,
            sensitivity: 0.001,
        }

    }
    fn get_screen(&self) -> Screen {
        let v_x = self.angle_vector_x*self.screen_size.x;
        let v_y = (v_x%self.vector_to_screen).norm()*self.screen_size.y;
        let pos = self.vector_to_screen*self.dist_to_screen-(v_x+v_y)*0.5;
        Screen {
            pos: pos,
            i: v_x.norm(),
            j: v_y.norm(),
        }
    }

    fn get_render_vectors(&self) -> Vec<Vec<Vector3>>{
        let sc = self.get_screen();
        let mut out: Vec<Vec<Vector3>> = vec![];
        let cell_x = sc.i*(self.screen_size.x/self.screen_resolution.0 as f32);
        let cell_y = sc.j*(self.screen_size.y/self.screen_resolution.1 as f32);
        for i in 0..self.screen_resolution.0 {
            out.push(vec![]);
            for j in 0..self.screen_resolution.1 {                
                let a = (sc.pos+(cell_x*(i as f32)+cell_y*(j as f32))).norm();
                //println!("{:?}", self.transform.rotation);
                out[i].push(self.transform.rotation*a);
                //println!("{}, {}, {} |", a.x, a.y, a.z);
            }
        }
        out
    }

    fn march(&self, dir_vector: Vector3, figure: &dyn Figure) -> (Option<Vector3>, usize) { //точка пересечения(если есть), кол-во итераций
        let mut ray = Vector3::new(0., 0., 0.);
        let mut a: Option<Vector3> = None;
        let mut j: usize = 0;
        for _k in 0..200 {
            let p = self.transform.position*ray;
            let dist = figure.get_distance(p);
            if dist < 0.01 { a = Some(p); break; } 
            else if ray.length() > 300. { a = None ;break; }
            ray = ray+dir_vector*dist;
            j += 1;
        }
        (a, j)
    }

    pub fn render(&self, figure: &dyn Figure, light: Vector3) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
        let mut pixels = image::ImageBuffer::from_pixel(self.screen_resolution.0 as u32,
             self.screen_resolution.1 as u32,
              image::Rgba([0,0,0, 255]));
        
        let render_vectors = self.get_render_vectors();
        for i in 0..self.screen_resolution.0 {
            for j in 0..self.screen_resolution.1 {
                let mut ray = Vector3::new(0., 0., 0.);
                pixels.put_pixel(i as u32, j as u32, image::Rgba([0, 0, 0, 255]));
                for _k in 0..200 {
                    let p = self.transform.position*ray;
                    let dist = figure.get_distance(p);
                    if dist < 0.01 {
                        let mut a = figure.get_color(p, light);
                        let l = _k as f32 * 0.5 + ray.length();
                        a = a - Vector3::new(l, l, l);
                        pixels.put_pixel(i as u32, j as u32, image::Rgba([a.x as u8, a.y as u8, a.z as u8, 255]));
                        break;
                    } 
                    else if ray.length() > 300. {break;}
                    ray = ray+render_vectors[i][j]*dist;
                }
            }
        }
        pixels
    }

    pub fn button_handler(&mut self, btn: &ButtonArgs) {
        if let Button::Keyboard(i) = btn.button  {    
            if let ButtonState::Press = btn.state {
                match i {
                    Key::W => self.move_vec.x = -1.,
                    Key::S => self.move_vec.x = 1.,
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

    pub fn mouse_move_handler(&mut self, x: &f64, y: &f64) {
        self.rot_vec = self.rot_vec + Vector3::new(0.0, -*x as f32 * self.sensitivity,  *y as f32 * self.sensitivity);
        self.rot_vec.z = self.rot_vec.z.min(3.14/2.);
        self.rot_vec.z = self.rot_vec.z.max(-3.14/2.);
        self.transform.rotation = Matrix4x4::new_rotation_matrix(self.rot_vec, None);
        self.transform.matrix = self.transform.size * self.transform.rotation * self.transform.position;
    }

    pub fn move_pos(&mut self) {
        self.transform.transform_matrix(Matrix4x4::new_rotation_matrix(self.rot_vec*(-1.), None) * self.move_vec.norm()*self.speed, Vector3::new(0., 0., 0.), Vector3::new(0., 0., 0.));
    }
}