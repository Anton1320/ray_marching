extern crate piston_window;
extern crate ray_marching;
extern crate image;
use piston_window::*;
use ray_marching::*;

struct SuperCube {
    sphere: Sphere,
    cube: Box,
    color: Vector3,
}

impl Figure for SuperCube {
    fn get_distance(&self, point: Vector3) -> f32 {
        self.cube.get_distance(point).max(-self.sphere.get_distance(point))
    }
}

fn main() {
    let cam = Camera {
        pos: Vector3::new(0., 0., 0.),
        screen_size: Vector3::new(4., 4., 0.),
        screen_resolution: (100, 100),
        dist_to_screen: 2.,
        vector_to_screen: Vector3::new(1., 0., 0.),
        angle_vector_x: Vector3::new(0., 0., 1.),
    };

    let light = Vector3::new(0., 0., 2.);
    
    let sphere = Sphere {
        center: Vector3::new(3.5, 0., 0.),
        r: 1.9,
        color: Vector3::new(255., 0., 0.),
    };

    let cube = Box::new
    (
        Vector3::new(3.5, 0., 0.),
        Vector3::new(0., 0., 0.),
        Vector3::new(1.4, 1.4, 1.4),
        Vector3::new(255., 255., 1.)
    );
    let mut super_cube = SuperCube {
        cube: cube,
        sphere: sphere,
        color: Vector3::new(255., 255., 1.),
    };
    let render_vectors = cam.get_render_vectors();

    let mut pixels = image::ImageBuffer::from_pixel(cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32, image::Rgba([0,0,0, 255]));
    let mut window: PistonWindow =
        WindowSettings::new("Ray marching", [cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        super_cube.cube.rotation.x += 1./40.;
        super_cube.cube.rotation.y += 1./45.;
        super_cube.cube.rotation.z += 1./35.;
        //sphere.center.z += 0.1;
        for i in 0..cam.screen_resolution.0 {
            for j in 0..cam.screen_resolution.1 {
                let mut ray = Vector3::new(0., 0., 0.);
                pixels.put_pixel(i as u32, j as u32, image::Rgba([0, 0, 0, 255]));
                for _k in 0..50 {
                    //ray.z = (ray.z-2.)%4.+2.;
                    let dist = super_cube.get_distance(cam.pos+ray);
                    if dist < 0.01 {
                        let a = super_cube.color*((super_cube.get_normal(cam.pos+ray)*(-1.)*(light-cam.pos-ray).norm()).powf(2.)+0.2);
                        pixels.put_pixel(i as u32, j as u32, image::Rgba([a.x as u8, a.y as u8, a.z as u8, 255]));
                        break;
                    } 
                    else if ray.length() > 10. {break;}
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