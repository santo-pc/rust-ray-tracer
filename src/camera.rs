
pub mod my {    
   use cgmath::Vector3; 
   use cgmath::prelude::*;
   use rust_math::trigonometry::*; 
    #[derive(Debug)]
    pub struct Camera {
        pub width: f32,
        pub heigh: f32,
        pub fov_y: f32,
        pub fov_x: f32,
        pub half_fov_y: f32,
        pub tan_half_fov_x: f32,
        pub tan_half_fov_y: f32,
        pub half_height: f32,
        pub half_width: f32,
        pub look_from: Vector3<f32>,
        pub look_at: Vector3<f32>,
        pub up: Vector3<f32>,
        pub w: Vector3<f32>,
        pub u: Vector3<f32>,
        pub v: Vector3<f32>,
    }


    impl Camera {
        pub fn pre_calc(&mut self) {
             // calc coordframe
            let a = self.look_from - self.look_at;
            let b = self.up;
            
            self.w = a.normalize();
            self.u = b.cross(self.w).normalize();
            self.v = self.w.cross(self.u);

            self.half_fov_y = self.half_fov_y / 2.0;

            let fovy_rads = self.fov_y * 3.14159265358979323846 / 180.0;

            // Calc vertical fov
            let aspect_ratio = self.width / self.heigh;
            self.fov_x = 2.0 * arccos(tan(fovy_rads / 2.0) * aspect_ratio);

            self.tan_half_fov_x = tan(self.fov_x / 2.0);
            self.tan_half_fov_y = tan(fovy_rads / 2.0);

            self.half_height = self.heigh / 2.0;
            self.half_width = self.width / 2.0;

        }

        pub fn from(look_from: Vector3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, fovy: f32) -> Self {
            let camera = Camera{
                width: 0.0,
                heigh: 0.0,
                fov_y: fovy,
                fov_x: 0.0,
                half_fov_y: 0.0,
                tan_half_fov_x: 0.0,
                tan_half_fov_y: 0.0,
                half_height: 0.0,
                half_width: 0.0,
                look_from,
                look_at,
                up,
                w: Vector3{x: 0.0, y: 0.0, z: 0.0},
                u: Vector3{x: 0.0, y: 0.0, z: 0.0},
                v: Vector3{x: 0.0, y: 0.0, z: 0.0}
            };
            return camera;
        }
    }
}

