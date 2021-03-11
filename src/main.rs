extern crate ray_marching;
use ray_marching::*;
fn main()
{
    let cam = Camera {
        pos: Vector3{x:0.,y:0.,z:0.},
        screen_size: Vector3{x:4., y:4., z:0.},
        screen_resolution: (10, 10),
        dist_to_screen: 1.,
        vector_to_screen: Vector3{x:1.,y:0.,z:0.},
        angle_vector_x: Vector3{x:0.,y:0.,z:1.}
    };
    
}