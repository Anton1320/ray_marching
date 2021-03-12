extern crate piston_window;
extern crate ray_marching;
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

    let light = Vector3::new(0., 0., 1.);
    
    let sphere = Sphere {
        center: Vector3::new(3.5, 0., 0.),
        r: 2.,
        color: Vector3::new(1., 0., 0.),
    };

    let cube = Box::new(Vector3::new(5., 0., 0.), Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.), Vector3::new(1., 1., 1.));

    let render_vectors = cam.get_render_vectors();

    let mut pixels: Vec<Vec<f32>> = vec![vec![0.; cam.screen_resolution.1]; cam.screen_resolution.0];

    println!("{}", 1);
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [cam.screen_resolution.0 as u32, cam.screen_resolution.1 as u32])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {

        for i in 0..cam.screen_resolution.0 {
            for j in 0..cam.screen_resolution.1 {
                let mut ray = Vector3::new(0., 0., 0.);
                for _k in 0..10 {
                    let dist = cube.get_distance(cam.pos+ray);
                    if dist < 0.01 {
                       pixels[i][j] = 1.;
                        //(cube.get_normal(cam.pos+ray)*(light-cam.pos-ray).norm()).powf(1.)+0.01;
                    } //else {println!("3");}
                    ray = ray+(render_vectors[i][j])*dist;
                }
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for i in 0..cam.screen_resolution.0 {
                for j in 0..cam.screen_resolution.1 {
                    let c = cube.color*pixels[i][j];
                    rectangle([c.x, c.y, c.z, 1.0],
                        [i as f64, j as f64, 1.0, 1.0],
                        context.transform,
                        graphics);
                }
            }
            println!("{}", 2);
        });
    }
}