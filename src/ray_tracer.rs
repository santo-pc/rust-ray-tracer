pub mod ray_tracer {

    use crate::camera::camera::Ray;
    use crate::shapes::shapes::AsGShape;
    use crate::Scene;
    use cgmath::{Vector3, Zero};

    #[derive(Debug)]
    pub enum TestHit {
        Hit(HitInfo),
        NoHit,
    }

    #[derive(Debug)]
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
        r: i32,
        g: i32,
        b: i32,
    }

    #[derive(Debug)]
    pub struct Image {
        width: i32,
        height: i32,
        image: Vec<Vec<Color>>,
    }

    impl Image {
        pub fn new(width: i32, height: i32) -> Image {
            Image {
                width,
                height,
                image: vec![vec![Color { r: 0, g: 0, b: 0 }; height as usize]; width as usize],
            }
        }

        pub fn convert_to_one_row_array(&self) -> Vec<i32> {
            // self.image.iter().flat_map(|row| row.iter()).cloned().collect()
            todo!("implement me")
        }
    }

    #[derive(Debug)]
    pub struct RayTracer {}

    impl RayTracer {
        pub fn ray_trace(&self, scene: &Scene, width: i32, height: i32) -> Image {
            let image = Image::new(width, height);
            let cam = scene.cams.get(0).unwrap();
            println!("Starting ray tracing with scene: {:?}", scene);

            for j in 0..=height {
                for i in 0..=width {
                    let x_mid = i as f32 + 0.5;
                    let y_mid = j as f32 + 0.5;

                    let ray = cam.ray_thru_pixel(x_mid, y_mid);
                    let hit = self.intersect(&ray, scene);
                    let _color = match hit {
                        TestHit::Hit(_) => Color { r: 1, g: 0, b: 0 },
                        TestHit::NoHit => Color { r: 0, g: 0, b: 0 },
                    };
                    // todo set color into image[i][j]
                }
            }

            return image;
        }

        fn intersect(&self, ray: &Ray, scene: &Scene) -> TestHit {
            let mut t_min = f32::MAX;
            let mut closest_intersection = HitInfo::new();
            closest_intersection.t_value = f32::MAX;

            for it in &scene.spheres {
                match it.intersection(ray) {
                    TestHit::Hit(test) => {
                        if test.t_value < t_min && test.t_value > 0.0 {
                            t_min = test.t_value;
                            closest_intersection = test;
                            closest_intersection.ray = ray.clone();
                        }
                    },
                    _ => (),
                }
            }
            return TestHit::Hit(closest_intersection);
        }
    }
}
