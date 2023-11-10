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
        t_value: f64,
        p: Vector3<f64>,
        n: Vector3<f64>,
        ray: Ray,
        color: Color,
    }

    impl HitInfo {
        pub fn from(
            t_value: f64,
            p: Vector3<f64>,
            n: Vector3<f64>,
            ray: Ray,
            color: Color,
        ) -> HitInfo {
            HitInfo { t_value, p, n, ray, color }
        }

        pub fn new() -> HitInfo {
            HitInfo {
                t_value: 0.0,
                p: Vector3::zero(),
                n: Vector3::zero(),
                ray: Ray::new(Vector3::zero(), Vector3::zero(), 0.0),
                color: Color { r: 0, g: 0, b: 0 },
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
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
        pub width: u32,
        pub height: u32,
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

        pub fn convert_to_one_row_array(&self) -> Vec<u8> {
            // let size: usize = (self.width * self.height * 3) as usize;
            // let mut result: Vec<u8> = vec![0u8; size as usize];
            // let mut row: usize = 0;
            // let mut col: usize = 0;
            // let mut i: usize = 0;
            // loop {
            //     result[i] = self.image[row][col].r.clone() as u8;
            //     result[i + 1] = self.image[row][col].g.clone() as u8;
            //     result[i + 2] = self.image[row][col].b.clone() as u8;
            //
            //     i += 3;
            //     col += 1;
            //
            //     if i as usize >= size {
            //         break;
            //     };
            //
            //     if col >= self.width as usize {
            //         col = 0;
            //         row += 1;
            //     }
            // }
            // return result;

            self.image
                .iter()
                .flatten()
                .into_iter()
                .map(|c| vec![c.r as u8, c.g as u8, c.b as u8])
                .flatten()
                .collect::<Vec<u8>>()
        }
    }

    #[derive(Debug)]
    pub struct RayTracer {}

    impl RayTracer {
        pub fn ray_trace(&self, scene: &Scene) -> Image {
            let cam = scene.cams.get(0).unwrap();
            let mut image = Image::new(cam.width, cam.height);
            // println!["Scene: {:?}", scene];

            for j in 0..cam.height {
                for i in 0..cam.width {
                    let x_mid = i as f64 + 0.5;
                    let y_mid = j as f64 + 0.5;

                    let ray = cam.ray_thru_pixel(x_mid, y_mid);
                    let hit = self.intersect(&ray, scene);

                    image.image[j as usize][i as usize] = match hit {
                        TestHit::Hit(info) => info.color,
                        TestHit::NoHit => Color { r: 0, g: 0, b: 0 },
                    };
                }
            }

            return image;
        }

        fn intersect(&self, ray: &Ray, scene: &Scene) -> TestHit {
            let mut t_min = f64::MAX;
            let mut closest_intersection = HitInfo::new();
            closest_intersection.t_value = f64::MAX;

            for it in &scene.triangles {
                match it.intersection(ray) {
                    TestHit::Hit(test) => {
                        // println!["Hit Triangle {:?}", ray];
                        if test.t_value < t_min && test.t_value > 0.0 {
                            t_min = test.t_value;
                            closest_intersection = test.clone();
                            closest_intersection.ray = ray.clone();
                        }
                    },
                    _ => (),
                }
            }

            for it in &scene.spheres {
                match it.intersection(ray) {
                    TestHit::Hit(test) => {
                        // println!["Hit sphere {:?}", ray];
                        if test.t_value < t_min && test.t_value > 0.0 {
                            t_min = test.t_value;
                            closest_intersection = test.clone();
                            closest_intersection.ray = ray.clone();
                        }
                    },
                    _ => (),
                }
            }

            if closest_intersection.t_value == f64::MAX {
                return TestHit::NoHit;
            } else {
                return TestHit::Hit(closest_intersection);
            }
        }
    }
}
