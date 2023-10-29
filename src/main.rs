pub mod camera;
mod ray_tracer;
pub mod shapes;

use crate::{camera::camera::Camera, shapes::shapes::Sphere};
use cgmath::Vector3;
use ray_tracer::ray_tracer::RayTracer;
use std::default;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Scene {
    cams: Vec<Camera>,
    spheres: Vec<Sphere>,
    triangles: Vec<Sphere>,
    vertices: Vec<Sphere>,
}

impl Scene {
    pub fn default() -> Scene {
        Scene { cams: vec![], spheres: vec![], triangles: vec![], vertices: vec![] }
    }
}

#[derive(Debug)]
struct RenderSettings {
    width: i32,
    height: i32,
    trace_depth: i32,
}

impl RenderSettings {
    pub fn default() -> RenderSettings {
        RenderSettings { width: 400, height: 300, trace_depth: 5 }
    }

    pub fn from(width: i32, height: i32) -> RenderSettings {
        RenderSettings { width, height, trace_depth: 5 }
    }
}

fn create_sample_cam() -> Camera {
    Camera::from(
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        90.0,
    )
}

fn main() -> io::Result<()> {
    // let scene =
    // read_scene("/Users/s.palacio-caro/Dev/rust/my-ray-tracer/src/scene1.test".to_string());
    let scene = Scene {
        cams: vec![create_sample_cam()],
        spheres: vec![create_sample_sphere()],
        triangles: vec![],
        vertices: vec![],
    };

    let tracer = RayTracer {};

    tracer.ray_trace(scene, width, height);

    Ok(())
}

// fn read_scene(file_path: String) -> Scene {
//     let file = File::open(file_path)?;
//     let reader = io::BufReader::new(file);
//     let mut scene = Scene::default();
//     let mut r_settings = RenderSettings::default();
//
//     // size 640 480
//     r_settings.width = 640;
//     r_settings.height = 480;
//
//     // camera -4 -4 4 1 0 0 0 1 0 30
//     scene.cams.push(Camera {
//         width: 640_f32,
//         height: 480_f32,
//         fov_y: 0.0_f32,
//         fov_x: 0.0_f32,
//         half_fov_y: 0.0_f32,
//         tan_half_fov_x: 0.0_f32,
//         tan_half_fov_y: 0.0_f32,
//         half_height: 0.0_f32,
//         half_width: 0.0_f32,
//         look_from: Vector3 { x: 0f32, y: 0f32, z: 0f32 },
//         look_at: Vector3 { x: 1f32, y: 0f32, z: 0f32 },
//         up: Vector3 { x: 0f32, y: 1f32, z: 0f32 },
//         w: Vector3 { x: 0f32, y: 0f32, z: 0f32 },
//         u: Vector3 { x: 0f32, y: 0f32, z: 0f32 },
//         v: Vector3 { x: 0f32, y: 0f32, z: 0f32 },
//     });
//
//     // sphere 1 2 3
//     scene.spheres.push(Sphere::from(1f32, 2f32, 3f32));
//
//     println!("RenderSettings {:?}", r_settings);
//
//     println!("Spheres: ");
//     for x in scene.spheres {
//         println!("{:?}", x);
//     }
//
//     return Ok(());
//
//     // for line in reader.lines() {
//     //     match line {
//     //         Ok(line) => {
//     //             let _list :Vec<&str> = line.trim().split(' ').collect();
//     //             let cmd = _list[0];
//     //             let _args = &_list[1..];
//     //
//     //             //println!("_list {:?}", _list);
//     //             if !cmd.starts_with("#") && !line.is_empty() {
//     //                 // println!("Commad: {}", cmd);
//     //                 // println!("args: {}", args.join(" ").to_string());
//     //                 if cmd == "size" {
//     //                     r_settings.width = _args[0].parse().unwrap();
//     //                     r_settings.height = _args[1].parse().unwrap();
//     //                 }
//     //                 else if cmd == "camera"{
//     //                     // TODO
//     //                     // scene.cams.push(
//     //                     //     Camera::from(Vector3::default() ,Vector3::default() , Vector3::default(), 0);
//     //                     // );
//     //                 }
//     //                 else if cmd == "sphere" {
//     //                     scene.spheres.push(Sphere::from(
//     //                         _args[0].parse::<f32>().unwrap(),
//     //                         _args[1].parse::<f32>().unwrap(),
//     //                         _args[2].parse::<f32>().unwrap()
//     //                         )
//     //                     );
//     //                 }
//     //             }
//     //
//     //         }
//     //         Err(err) => {
//     //             eprintln!("Error reading line: {}", err);
//     //         }
//     //     }
// }
//
fn handle_cmd(cmd: &str, args: Vec<&str>) {}
