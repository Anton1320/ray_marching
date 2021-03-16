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

struct SuperSphere {
    s:Sphere,
}

impl Figure for SuperSphere {
    fn get_distance(&self, point:Vector3) -> f32 {
        let q = ((point+Vector3::new(1.5, 1.5, 1.5)) % 3.)-Vector3::new(1.5, 1.5, 1.5);
        self.s.get_distance(q)
    }
}



fn main() {
    let mut cam = Camera::new(
        Vector3::new(-3., 0., 0.),
        Vector3::new(1., 1., 0.),
        (50, 50),
        0.5,
        Vector3::new(1., 0., 0.),
        Vector3::new(0., 0., 1.),   
    );

    let light = Vector3::new(0., 0., 2.);
    
    let sphere = Sphere {
        center: Vector3::new(1., 1., 1.),
        r: 0.5,
        color: Vector3::new(0., 0., 255.),
    };
    let super_sphere  = SuperSphere {s:sphere};
    /*let cube = Box::new
    (
        Vector3::new(3.5, 0., 0.),
        Vector3::new(0., 0., 0.),
        Vector3::new(1.4, 1.4, 1.4),
        Vector3::new(255., 255., 1.)
    );*/
    //let mut super_cube = SuperCube { cube: cube, sphere: sphere, color: Vector3::new(255., 255., 1.), };
    let render_vectors = cam.get_render_vectors();
    let mut pixels = image::ImageBuffer::from_pixel(cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32, image::Rgba([0,0,0, 255]));
    let mut window: PistonWindow =
        WindowSettings::new("Ray marching", [cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32])
        .exit_on_esc(true).build().unwrap();

    
    
    while let Some(event) = window.next() {
        //super_cube.cube.rotation.x += 1./40.;
        //super_cube.cube.rotation.y += 1./45.;
        //super_cube.cube.rotation.z += 1./35.;
        //sphere.center.z += 0.1;
        if let Event::Input(i) = &event{
            if let Input::Button(j) = i {
                cam.button_handler(j);
            }
        }
        cam.move_pos();
        for i in 0..cam.screen_resolution.0 {
            for j in 0..cam.screen_resolution.1 {
                let mut ray = Vector3::new(0., 0., 0.);
                pixels.put_pixel(i as u32, j as u32, image::Rgba([0, 0, 0, 255]));
                for _k in 0..100 {
                    let p = cam.pos+ray;
                    let dist = super_sphere.get_distance(p);
                    if dist < 0.01 {
                        let a =  super_sphere.s.color*((super_sphere.get_normal(cam.pos+ray)*(light-cam.pos-ray).norm()).powf(1.)+0.1);
                        pixels.put_pixel(i as u32, j as u32, image::Rgba([a.x as u8, a.y as u8, a.z as u8, 255]));
                        break;
                    } 
                    else if ray.length() > 20. {break;}
                    ray = ray+(render_vectors[i][j])*dist;
                }
            }
        }
        let tex = Texture::from_image(
            &mut window.create_texture_context(),
            &pixels,
            &piston_window::TextureSettings::new())
            .unwrap();

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);
            image(&tex, context.transform, graphics);
        });
    }
}