extern crate piston_window;
extern crate ray_marching;
extern crate image;
use piston_window::*;
use ray_marching::*;

fn main() {
    let cam = Camera {
        pos: Vector3::new(0., 0., 0.),
        screen_size: Vector3::new(4., 4., 0.),
        screen_resolution: (150,150),
        dist_to_screen: 2.,
        vector_to_screen: Vector3::new(1., 0., 0.),
        angle_vector_x: Vector3::new(0., 0., 1.),
    };

    let light = Vector3::new(0., 0., 2.);
    
    let sphere = Sphere {
        center: Vector3::new(3.5, 0., 0.),
        r: 2.,
        color: Vector3::new(1., 0., 0.),
    };

    let mut cube = Box::new(Vector3::new(3.5, 0., 0.), Vector3::new(0., 3.14159265/4., 3.14159265/4.), Vector3::new(1., 1., 1.), Vector3::new(1., 1., 1.));

    let render_vectors = cam.get_render_vectors();

    let mut pixels: Vec<Vec<f32>> = vec![vec![0.; cam.screen_resolution.1]; cam.screen_resolution.0];

    let mut frame = 0;

    //println!("{}", 1);
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        frame += 1;
        //cube.rotation.x += (frame as f32)/150.;
        //cube.rotation.y += (frame as f32)/200.;
        //cube.rotation.z += (frame as f32)/70.;
        for i in 0..cam.screen_resolution.0 {
            for j in 0..cam.screen_resolution.1 {
                let mut ray = Vector3::new(0., 0., 0.);
                pixels[i][j] = 0.;
                for _k in 0..100 {
                    let dist = sphere.get_distance(cam.pos+ray);
                    if dist < 0.01 {
                       pixels[i][j] = (sphere.get_normal(cam.pos+ray)*(light-cam.pos-ray).norm()).powf(2.)+0.1;
                       break;
                    } //else {println!("3");}
                    ray = ray+(render_vectors[i][j])*dist;
                }
            }
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);
            for i in 0..cam.screen_resolution.0 {
                for j in 0..cam.screen_resolution.1 {
                    let c = sphere.color*pixels[i][j];
                    rectangle([c.x, c.y, c.z, 1.0],
                        [i as f64, j as f64, 1.0, 1.0],
                        context.transform,
                        graphics);
                }
            }
        });
    }
}