pub mod camera {
    use std::f64::consts::PI;

    use cgmath::prelude::*;
    use cgmath::Vector3;
    use rust_math::trigonometry::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Camera {
        pub width: u32,
        pub height: u32,
        pub fov_y: f64,
        pub fov_x: f64,
        pub half_fov_y: f64,
        pub tan_half_fov_x: f64,
        pub tan_half_fov_y: f64,
        pub half_height: f64,
        pub half_width: f64,
        pub look_from: Vector3<f64>,
        pub look_at: Vector3<f64>,
        pub up: Vector3<f64>,
        pub w: Vector3<f64>,
        pub u: Vector3<f64>,
        pub v: Vector3<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Ray {
        pub o: Vector3<f64>,
        pub d: Vector3<f64>,
        pub t: f64,
    }

    impl Ray {
        pub fn new(o: Vector3<f64>, d: Vector3<f64>, t: f64) -> Ray {
            Ray { o, d, t }
        }
    }

    impl Camera {
        pub fn new(
            width: u32,
            height: u32,
            look_from: Vector3<f64>,
            look_at: Vector3<f64>,
            up: Vector3<f64>,
            fov_y: f64,
        ) -> Self {
            let mut camera = Camera {
                width,
                height,
                fov_y,
                fov_x: 45.0,
                half_fov_y: 0.0,
                tan_half_fov_x: 0.0,
                tan_half_fov_y: 0.0,
                half_height: 0.0,
                half_width: 0.0,
                look_from,
                look_at,
                up,
                w: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                u: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                v: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            };
            camera.pre_calc();
            camera
        }

        fn pre_calc(&mut self) {
            // calc coordframe
            let a = self.look_from - self.look_at;
            let b = self.up;

            self.w = a.normalize();
            self.u = b.cross(self.w).normalize();
            self.v = self.w.cross(self.u);

            self.half_fov_y = self.fov_y / 2.0;

            // Calc vertical fov
            let fovy_rads = self.fov_y * PI / 180.0;
            let aspect_ratio = (self.width / self.height) as f64;

            self.fov_x = 2.0 * (((fovy_rads / 2.0).tan()) * aspect_ratio).atan();
            // self.fov_x = 2.0 * arctan(tan(fovy_rads / 2.0) * aspect_ratio);

            // pre cal tan of half fovs
            self.tan_half_fov_x = (self.fov_x / 2.0).tan();
            // self.tan_half_fov_x = tan(self.fov_x / 2.0);

            self.tan_half_fov_y = (fovy_rads / 2.0).tan();
            // self.tan_half_fov_y = tan(fovy_rads / 2.0);

            self.half_height = self.height as f64 / 2.0;
            self.half_width = self.width as f64 / 2.0;
        }

        pub fn ray_thru_pixel(&self, x: f64, y: f64) -> Ray {
            let alpha = self.tan_half_fov_x * ((x - self.half_width) / self.half_width);
            let beta = self.tan_half_fov_y * ((self.half_height - y) / self.half_height);

            let o = self.look_from;
            let d = ((alpha * self.u) + (-beta * self.v) - self.w).normalize();

            let t = 10000.0;

            let ray = Ray::new(o, d, t);
            // println!["Generated ray: {:?}", ray];
            return ray;
        }

        // pub fn eye_ray(self, x: i32, y: i32, img_width: i32, img_height: i32) -> Ray {
        //     let w_dir = self.w;
        //     let u_dir = self.u;
        //     let v_dir = self.v;
        //
        //     let aspect_ratio: f64 = img_width as f64 / img_height as f64;
        //
        //     let top = tan(self.fov_y / 2.0);
        //     let right = aspect_ratio * top;
        //     let bottom = -top;
        //     let left = -right;
        //
        //     // transform x and y into camera space
        //     let im_plane_u_pos = left + (right - left) * (x as f64 / img_width as f64);
        //     let im_plane_v_pos = bottom + (top - bottom) * (y as f64 / img_height as f64);
        //
        //     return Ray::new(
        //         self.look_from,
        //         ((im_plane_u_pos * u_dir) + im_plane_v_pos * v_dir - w_dir).normalize(),
        //         99999.0,
        //     );
        // }
    }
}
