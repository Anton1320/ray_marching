extern crate piston_window;
extern crate ray_marching;
extern crate image;
use piston_window::*;
use ray_marching::*;

struct Fractal<'a> {
    pub color: Vector3,
    pub transform: Transform,
    pub children: Vec<&'a mut (dyn Figure<'a> + 'a)>,
}

impl<'a> Figure<'a> for Fractal<'a> {
    fn get_distance(&self, point:Vector3) -> f32 {
        let mut z = point;
        let mut dr = 1.0;
        let mut r = 0.0;
        for _i in 0..5 {
            r = z.length();
            if r>5. {break};
            
            // convert to polar coordinates
            let mut theta = (z.z/r).acos();
            let mut phi = z.y.atan2(z.x);
            dr =  r.powf(5.)*5.*dr + 1.0;
            
            // scale and rotate the point
            let zr = r.powf(5.);
            theta = theta*5.;
            phi = phi*5.;
            
            // convert back to cartesian coordinates
            z = Vector3::new(theta.sin()*phi.cos(), phi.sin()*theta.sin(), theta.cos())*zr;
            z = z+point;
	    }
	    return 0.5*r.log2() *r/dr;
    }
    fn get_transform(&self) -> &Transform { &self.transform }
    fn get_mut_transform(&mut self) -> &mut Transform { &mut self.transform }
    fn get_figure_color(&self) -> Vector3 {self.color}
    fn get_children(&self) -> &Vec<&'a mut (dyn Figure<'a> + 'a)>{&self.children}
    fn get_mut_children(&mut self) -> &mut Vec<&'a mut (dyn Figure<'a> + 'a)> { &mut self.children }
}

fn main() {
    let mut cam = Camera::new(
        Vector3::new(0., 0., 0.),
        Vector3::new(0.5, 0.5, 0.),
        (200, 200),
        0.5,  
    );
    let f = Fractal{
        color: Vector3::new(255.,0.,255.),
         transform: Transform::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 0.),Vector3::new(1., 1., 1.)),
          children: vec![]
        };
    //let light = Vector3::new(0., -3., 2.);
    let light = Vector3::new(5., -10., 0.);
    let mut sphere = Sphere::new(Vector3::new(-3., 0., 0.), Vector3::new(0., 0., 0.), 1.5, Vector3::new(0., 0., 255.), None);
    //let super_sphere  = SuperSphere {s:sphere};
    let mut cube = Box::new
    (
        Vector3::new(3., 0., 0.),
        Vector3::new(0., 0., 0.),
        Vector3::new(1., 1., 1.),
        Vector3::new(255., 255., 1.),
        None
    );

    let mut plane = Plane {
        transform:Transform::new(Vector3::new(0., -5., 0.), Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.)),
        color: Vector3::new(255., 0., 0.),
        children: vec![],
    };

    let mut torus = Torus::new(Vector3::new(0., 2., 0.), Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.),
    1., 0.4, Vector3::new(0., 255., 0.), None);

    let mut scene = Folder {
        children: vec![&mut cube,  &mut plane, &mut sphere, &mut torus],
        transform: Transform::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.)),
    };

    //let mut super_cube = SuperCube { cube: cube, sphere: sphere, color: Vector3::new(255., 255., 1.), };

    let mut pixels = image::ImageBuffer::from_pixel(cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32, image::Rgba([0,0,0, 255]));
    let mut window: PistonWindow =
        WindowSettings::new("Ray marching", [cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();    
    window.set_capture_cursor(true);
    window.set_max_fps(60);
    let mut events = window.events;

    while let Some(event) = events.next(&mut window) {
        //println!("1 {:?}", cam.transform.rotation);
        if let Event::Input(i) = &event{
            if let Input::Button(j) = i {
                cam.button_handler(j);
            }
            else if let Input::Move(j) = i
            {
                if let Motion::MouseRelative(x, y) = j{
                    
                    cam.mouse_move_handler(x, y);
                }
            }
        }

        cam.move_pos();
        let tex = Texture::from_image(
            &mut window.create_texture_context(),
            &pixels,
            &TextureSettings::new())
            .unwrap();
        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);
            scene.children[0].change_transform(Vector3::new(0.0, 0., 0.), Vector3::new(0.03, 0.06, 0.09), Vector3::new(0., 0.0, 0.0));
            scene.children[3].change_transform(Vector3::new(0.0, 0., 0.), Vector3::new(0.1, 0., 0.2), Vector3::new(0., 0.0, 0.0));
            scene.change_transform(Vector3::new(0.0, 0., 0.), Vector3::new(0., 0.1, 0.), Vector3::new(0., 0.0, 0.0));
            pixels = cam.render(&scene, light);
            image(&tex, context.transform, graphics);
        }) ;
        
    }
}