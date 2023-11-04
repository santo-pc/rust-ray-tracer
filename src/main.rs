// TODO:
// - Review sphere math (precalc and intersection) and add tests
// - Add scene parser for test input files
// - Add shading

pub mod camera;
mod ray_tracer;
pub mod shapes;

use crate::ray_tracer::ray_tracer::Color;
use crate::{camera::camera::Camera, shapes::shapes::Sphere};
use cgmath::{Matrix4, One, Rad, SquareMatrix, Vector3, Zero};
use ray_tracer::ray_tracer::{Image, RayTracer};
use std::default;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Scene {
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
    width: u32,
    height: u32,
    trace_depth: i32,
    output_file: String,
}

impl RenderSettings {
    pub fn default() -> RenderSettings {
        RenderSettings { width: 400, height: 300, trace_depth: 5, output_file: "".to_string() }
    }

    pub fn from(width: u32, height: u32) -> RenderSettings {
        RenderSettings { width, height, trace_depth: 5, output_file: "".to_string() }
    }
}

fn create_sample_sphere() -> Sphere {
    Sphere::from(-0.5, 1.0, -0.5, 0.15, Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)))
}

fn create_sample_cam() -> Camera {
    Camera::new(
        800,
        600,
        Vector3 { x: -2.0, y: -2.0, z: 2.0 },
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 1.0, y: 1.0, z: 2.0 },
        60.0,
    )
}

fn main() -> io::Result<()> {
    let scene = read_scene("src/scene3.test".to_string());
    // image::save_buffer("image.png", buffer, 800, 600, image::ColorType::Rgb8).unwrap();

    // println!("Starting to build scene");
    // let scene = Scene {
    //     cams: vec![create_sample_cam()],
    //     spheres: vec![create_sample_sphere()],
    //     triangles: vec![],
    //     vertices: vec![],
    // };
    //
    let tracer = RayTracer {};

    // tracer.ray_trace(&scene, 4, 2);
    // let result = &tracer.test_scene(400, 400).convert_to_one_row_array();
    let result = &tracer.ray_trace(&scene).convert_to_one_row_array();

    image::save_buffer(
        "image.png",
        result,
        scene.cams[0].width,
        scene.cams[0].height,
        image::ColorType::Rgb8,
    )
    .unwrap();
    Ok(())
}

fn read_scene(file_path: String) -> Scene {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut scene = Scene::default();
    let mut r_settings = RenderSettings::default();

    let mut transfstack: Vec<Matrix4<f32>> = vec![Matrix4::one()];
    let mut inverse_transfstack: Vec<Matrix4<f32>> = vec![Matrix4::one()];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if !line.starts_with("#") && !line.is_empty() {
                    let _list: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
                    let cmd = _list[0];
                    let _args: Vec<f32> =
                        _list[1..].iter().map(|a| a.parse::<f32>().unwrap()).collect();

                    println!("Line: {}", line);

                    match cmd {
                        "size" => {
                            r_settings.width = _args[0] as u32;
                            r_settings.height = _args[1] as u32;
                        },
                        "output" => r_settings.output_file = _args[0].to_string(),
                        "maxdepth" => r_settings.trace_depth = _args[0] as i32,
                        "camera" => scene.cams.push(Camera::new(
                            r_settings.width.clone(),
                            r_settings.height.clone(),
                            Vector3::new(_args[0], _args[1], _args[2]),
                            Vector3::new(_args[3], _args[4], _args[5]),
                            Vector3::new(_args[6], _args[7], _args[8]),
                            _args[9],
                        )),
                        "sphere" => {
                            scene.spheres.push(Sphere::from(
                                _args[0],
                                _args[1],
                                _args[2],
                                _args[3],
                                transfstack.last().unwrap().clone(),
                            ));
                        },

                        // TRANSFORMS
                        "translate" => {
                            let translation = &Matrix4::from_translation(Vector3::new(
                                _args[0], _args[1], _args[2],
                            ));

                            right_multiply(&translation, &mut transfstack);
                            left_multiply(&translation.invert().unwrap(), &mut inverse_transfstack);
                        },
                        "scale" => {
                            let scale =
                                &Matrix4::from_nonuniform_scale(_args[0], _args[1], _args[2]);

                            right_multiply(&scale, &mut transfstack);
                            left_multiply(&scale.invert().unwrap(), &mut inverse_transfstack);
                        },
                        "rotate" => {
                            let axis = Vector3::new(_args[0], _args[1], _args[2]);
                            // read degrees and pass as rads
                            let theta = Rad(_args[4] * PI / 180.0);

                            let scale = &Matrix4::from_axis_angle(axis, theta);

                            right_multiply(&scale, &mut transfstack);
                            left_multiply(&scale.invert().unwrap(), &mut inverse_transfstack);
                        },
                        "pushTransform" => {
                            transfstack.push(Matrix4::zero());
                            ()
                        },

                        "popTransform" => {
                            transfstack.pop();
                            ()
                        },

                        _ => println!("Neglecting cmd {}", cmd),
                    };
                } else {
                    println!("Was empty line or comment");
                }
            },
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            },
        }
    }

    return scene;
}

fn right_multiply(m: &Matrix4<f32>, transfstack: &mut Vec<Matrix4<f32>>) {
    let top = transfstack.pop().unwrap();
    transfstack.push(top * m);
}

fn left_multiply(inverse_m: &Matrix4<f32>, inverse_transfstack: &mut Vec<Matrix4<f32>>) {
    let top = inverse_transfstack.pop().unwrap();
    inverse_transfstack.push(inverse_m * top);
}
