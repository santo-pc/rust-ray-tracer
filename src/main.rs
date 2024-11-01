pub mod camera;
mod ray_tracer;
pub mod shapes;

use crate::{
    camera::camera_view::Camera,
    shapes::shape_components::{Sphere, Triangle},
};
use cgmath::{Matrix4, One, Rad, SquareMatrix, Vector3, Vector4};
use log::{debug, error, info, warn};
use ray_tracer::tracer::RayTracer;

use std::time::Instant;
use std::{
    env,
    f64::consts::PI,
    fs::File,
    io::{self, BufRead},
};

trait ConvertToVector4<T> {
    fn to_vector4(&self) -> Vector4<T>;
}

impl<T> ConvertToVector4<T> for Vector3<T>
where
    T: From<f32> + Copy,
{
    fn to_vector4(&self) -> Vector4<T> {
        Vector4::new(self.x, self.y, self.z, T::from(1.0))
    }
}

#[derive(Debug)]
pub struct Scene {
    cams: Vec<Camera>,
    spheres: Vec<Sphere>,
    triangles: Vec<Triangle>,
    vertices: Vec<Vector3<f64>>,
    settings: RenderSettings,
}

impl std::default::Default for Scene {
    fn default() -> Self {
        Self {
            cams: vec![],
            spheres: vec![],
            triangles: vec![],
            vertices: vec![],
            settings: RenderSettings::default(),
        }
    }
}

#[derive(Debug)]
struct RenderSettings {
    pub width: u32,
    pub height: u32,
    trace_depth: i32,
    pub output_file: String,
}

impl RenderSettings {
    pub fn default() -> RenderSettings {
        RenderSettings {
            width: 400,
            height: 300,
            trace_depth: 5,
            output_file: "image_out.png".to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    log4rs::init_file("src/log4rs.yml", Default::default()).unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Input file is mandatory");
    }

    let file_name = &args[1];

    let file_path = "src/".to_string() + file_name;
    let output_file = "output_".to_string() + file_name + ".png";
    let scene = read_scene(file_path);

    let now = Instant::now();
    let image = RayTracer {}.ray_trace_par(&scene);
    info!("Finished render in {} milliseconds", now.elapsed().as_millis());

    image::save_buffer(
        output_file.clone(),
        &image.convert_to_one_row_array(),
        image.width,
        image.height,
        image::ColorType::Rgb8,
    )
    .unwrap();

    info!("Saved image to file {}s", output_file);
    Ok(())
}

fn read_scene(file_path: String) -> Scene {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut scene = Scene::default();

    let mut transfstack: Vec<Matrix4<f64>> = vec![Matrix4::one()];
    let mut inverse_transfstack: Vec<Matrix4<f64>> = vec![Matrix4::one()];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if !line.starts_with('#') && !line.is_empty() {
                    info!("Line: {}", line);
                    let _list: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
                    let cmd = _list[0];
                    let args: Vec<f64> =
                        _list[1..].iter().map(|a| a.parse::<f64>().unwrap()).collect();

                    match cmd {
                        "size" => handle_size(&mut scene, args[0] as u32, args[1] as u32),
                        "output" => scene.settings.output_file = args[0].to_string(),
                        "maxdepth" => scene.settings.trace_depth = args[0] as i32,
                        "camera" => scene.cams.push(create_camera(
                            scene.settings.width,
                            scene.settings.height,
                            &args,
                        )),

                        // GEOMETRY
                        "sphere" => {
                            scene.spheres.push(create_sphere(&args, *transfstack.last().unwrap()))
                        },
                        "maxverts" => {
                            // scene.vertices
                        },
                        "vertex" => {
                            scene.vertices.push(Vector3::new(args[0], args[1], args[2]));
                        },
                        "tri" => {
                            scene.triangles.push(create_triangle(args, &scene));
                        },

                        // TRANSFORMS
                        "translate" => {
                            let translation =
                                Matrix4::from_translation(Vector3::new(args[0], args[1], args[2]));
                            // info!("Generated translation matrix: {:?} ", translation);

                            right_multiply(translation, &mut transfstack);
                            left_multiply(translation.invert().unwrap(), &mut inverse_transfstack);
                            // info!("Stack state:  {:?}", transfstack.iter().copied().rev());
                        },
                        "scale" => {
                            let scale = Matrix4::from_nonuniform_scale(args[0], args[1], args[2]);

                            right_multiply(scale, &mut transfstack);
                            left_multiply(scale.invert().unwrap(), &mut inverse_transfstack);
                            // info!("Stack state:  {:?}", transfstack.iter());
                        },
                        "rotate" => {
                            let axis = Vector3::new(args[0], args[1], args[2]);
                            // read degrees and pass as rads
                            let theta = Rad(args[3] * PI / 180.0);

                            let scale = Matrix4::from_axis_angle(axis, theta);

                            right_multiply(scale, &mut transfstack);
                            left_multiply(scale.invert().unwrap(), &mut inverse_transfstack);
                            // info!("Stack state:  {:?}", transfstack.iter().copied().rev());
                        },
                        "pushTransform" => {
                            transfstack.push(*transfstack.last().unwrap());
                        },

                        "popTransform" => {
                            transfstack.pop();
                        },

                        _ => warn!("Neglecting cmd {}", cmd),
                    };
                } else {
                    warn!("Was empty line or comment");
                }
            },
            Err(err) => {
                error!("Error reading line: {}", err);
            },
        }
    }

    scene
}

fn create_triangle(args: Vec<f64>, scene: &Scene) -> Triangle {
    let vert_indexes = vec![args[0] as usize, args[1] as usize, args[2] as usize];
    let a = scene.vertices[vert_indexes[0]].to_vector4();
    let b = scene.vertices[vert_indexes[1]].to_vector4();
    let c = scene.vertices[vert_indexes[2]].to_vector4();
    Triangle::new(vert_indexes, a, b, c)
}

fn create_sphere(args: &[f64], transform: Matrix4<f64>) -> Sphere {
    Sphere::from(args[0], args[1], args[2], args[3], transform)
}

fn create_camera(width: u32, height: u32, _args: &[f64]) -> Camera {
    Camera::new(
        width,
        height,
        Vector3::new(_args[0], _args[1], _args[2]),
        Vector3::new(_args[3], _args[4], _args[5]),
        Vector3::new(_args[6], _args[7], _args[8]),
        _args[9],
    )
}

fn handle_size(scene: &mut Scene, width: u32, height: u32) {
    scene.settings.width = width;
    scene.settings.height = height;
}

fn right_multiply(m: Matrix4<f64>, transfstack: &mut Vec<Matrix4<f64>>) {
    let top = transfstack.pop().unwrap();
    transfstack.push(top * m);
}

fn left_multiply(inverse_m: Matrix4<f64>, inverse_transfstack: &mut Vec<Matrix4<f64>>) {
    let top = inverse_transfstack.pop().unwrap();
    inverse_transfstack.push(inverse_m * top);
}
