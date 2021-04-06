extern crate piston_window;
extern crate ray_marching;
extern crate image;
use piston_window::*;
use ray_marching::*;

/*struct SuperCube { sphere: Sphere, cube: Box, color: Vector3,}

impl Figure for SuperCube {
    fn get_distance(&self, point: Vector3) -> f32 {
        self.cube.get_distance(point).max(-self.sphere.get_distance(point))
    }
}*/
/*
struct SuperSphere {
    s:Sphere,
}

impl Figure for SuperSphere {
    fn get_distance(&self, point:Vector3) -> f32 {
        let q = ((point+Vector3::new(3., 3., 3.)) % 6.)-Vector3::new(3., 3., 3.);
        self.s.get_distance(q)
    }
}*/


fn main() {
    let mut cam = Camera::new(
        Vector3::new(0., 0., 0.),
        Vector3::new(1., 1., 0.),
        (200, 200),
        0.5,  
    );

    //let light = Vector3::new(0., -3., 2.);
    let light = Vector3::new(5., -10., 5.);
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
        y: 5.,
        transform:Transform::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.)),
        color: Vector3::new(255., 0., 0.),
        children: vec![],
    };

    let mut scene = Folder {
        children: vec![&mut cube,  &mut plane, &mut sphere],
        transform: Transform::new(Vector3::new(0., 0., 0.), Vector3::new(0., 3.14, 0.), Vector3::new(1., 1., 1.)),
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
            //scene.children[0].change_transform(Vector3::new(0.0, 0., 0.), Vector3::new(0.03, 0.06, 0.09), Vector3::new(0., 0.0, 0.0));
            //scene.children[2].change_transform(Vector3::new(0.0, 0., 0.), Vector3::new(0.03, 0.06, 0.09), Vector3::new(0., 0.0, 0.0));
            scene.change_transform(Vector3::new(0.0, 0., 0.), Vector3::new(0.00, 0.1, 0.), Vector3::new(0., 0.0, 0.0));
            pixels = cam.render(&scene, light);
            
            image(&tex, context.transform, graphics);
        }) ;
        
    }
}