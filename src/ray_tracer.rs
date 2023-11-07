pub mod ray_tracer {

    use std::fmt::{self, Display};

    use crate::camera::camera::Ray;
    use crate::shapes::shapes::AsGShape;
    use crate::Scene;
    use cgmath::{Vector3, Zero};

    #[derive(Debug, Clone, Copy)]
    pub enum TestHit {
        Hit(HitInfo),
        NoHit,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct HitInfo {
        t_value: f32,
        p: Vector3<f32>,
        n: Vector3<f32>,
        ray: Ray,
    }

    impl HitInfo {
        pub fn from(t_value: f32, p: Vector3<f32>, n: Vector3<f32>, ray: Ray) -> HitInfo {
            HitInfo { t_value, p, n, ray }
        }

        pub fn new() -> HitInfo {
            HitInfo {
                t_value: 0.0,
                p: Vector3::zero(),
                n: Vector3::zero(),
                ray: Ray::new(Vector3::zero(), Vector3::zero(), 0.0),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Color {
        pub r: i32,
        pub g: i32,
        pub b: i32,
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[r{}g{}b{}]", self.r, self.g, self.b)
        }
    }

    #[derive(Debug)]
    pub struct Image {
        width: u32,
        height: u32,
        pub image: Vec<Vec<Color>>,
    }

    impl Image {
        pub fn new(width: u32, height: u32) -> Image {
            Image {
                width,
                height,
                // matrix hxw
                image: vec![vec![Color { r: 0, g: 0, b: 0 }; width as usize]; height as usize],
            }
        }

        pub fn print_matrix(&self) -> () {
            self.image.iter().for_each(|col| {
                col.iter().for_each(|cell| print!("{}", cell));
                println!("");
            });
        }

        pub fn convert_to_one_row_array(&self) -> Vec<u8> {
            // More functional approach
            return self
                .image
                .iter()
                .flatten()
                .into_iter()
                .map(|c| vec![c.r as u8, c.g as u8, c.b as u8])
                .flatten()
                .collect::<Vec<u8>>();
        }
    }

    #[derive(Debug)]
    pub struct RayTracer {}

    impl RayTracer {
        pub fn ray_trace(&self, scene: &Scene) -> Image {
            let cam = scene.cams.get(0).unwrap();
            let mut image = Image::new(cam.width, cam.height);

            // for sphere in &scene.spheres {
            //     println!["Sphere transform: {:?}", sphere.as_g_shape()];
            // }
            //
            // println!("Starting ray tracing with scene: {:?}", scene);

            for j in 0..cam.height {
                for i in 0..cam.width {
                    let x_mid = i as f32 + 0.5;
                    let y_mid = j as f32 + 0.5;

                    let ray = cam.ray_thru_pixel(x_mid, y_mid);
                    let hit = self.intersect(&ray, scene);
                    let _color = match hit {
                        TestHit::Hit(_) => {
                            println!["Hit with ray: {:?}", ray];
                            Color { r: 255, g: 0, b: 0 }
                        },
                        TestHit::NoHit => Color { r: 0, g: 0, b: 0 },
                    };

                    image.image[j as usize][i as usize] = _color;

                    println![
                        "Progress: {}",
                        (j as f64 + i as f64) / (cam.width as f64 + cam.height as f64) * 100.0
                    ];
                }
            }

            // image.print_matrix();
            return image;
        }

        fn intersect(&self, ray: &Ray, scene: &Scene) -> TestHit {
            let mut t_min = f32::MAX;
            let mut closest_intersection = HitInfo::new();
            closest_intersection.t_value = f32::MAX;

            for it in &scene.spheres {
                println!["Testing Ray: {:?} with Shere: {:?}, {:?}, {:?}", ray, it.x, it.y, it.z];
                match it.intersection(ray) {
                    TestHit::Hit(test) => {
                        println!["There was a hit with ray: {:?}", ray];
                        if test.t_value < t_min && test.t_value > 0.0 {
                            t_min = test.t_value;
                            closest_intersection = test.clone();
                            closest_intersection.ray = ray.clone();
                        }
                    },
                    _ => (),
                }
            }

            if closest_intersection.t_value == f32::MAX {
                return TestHit::NoHit;
            } else {
                return TestHit::Hit(closest_intersection);
            }
        }

        // pub fn test_scene(&self, width: u32, height: u32) -> Image {
        //     let mut image = Image::new(width, height);
        //
        //     for j in 0..height {
        //         for i in 0..width {
        //             if j >= (height / 2) - 20
        //                 && i >= (width / 2) - 20
        //                 && j <= (height / 2) + 20
        //                 && i <= (width / 2) + 20
        //             {
        //                 image.image[j as usize][i as usize] = Color { r: 255, g: 0, b: 0 };
        //             } else {
        //                 image.image[j as usize][i as usize] = Color { r: 0, g: 0, b: 0 };
        //             }
        //         }
        //     }
        //     println!(
        //         "Size of image: {}x{}x3={}",
        //         image.image.len(),
        //         image.image[0].len(),
        //         image.image.len() * image.image[0].len() * 3
        //     );
        //     return image;
        // }
    }
}
