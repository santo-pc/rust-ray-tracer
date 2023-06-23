
pub mod camera;
use crate::camera::my::Camera;
use cgmath::Vector3; 

fn main() {
    let mut c = Camera::from(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(10.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        10.0
    );

    println!("{:?}", c);
    c.pre_calc();
    println!("{:?}", c);

}
