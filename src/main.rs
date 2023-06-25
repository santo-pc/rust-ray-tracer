
pub mod camera;
pub mod shapes;
use crate::{camera::camera::Camera, shapes::shapes::Sphere};
use cgmath::Vector3; 

fn main() {
    let mut c = Camera::from(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(10.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        10.0
    );

    println!("{:#?}", c);
    c.pre_calc();
    println!("{:#?}", c);
    

    println!("{:#?}", Vector3::new(10.0, 10.0, 10.0) * 2.0);
    println!("{:#?}", Sphere::default());

}
