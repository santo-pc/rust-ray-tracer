
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
            println!("1");
             // calc coordframe
            let a = self.look_from - self.look_at;
            let b = self.up;
            
            self.w = a.normalize();
            self.u = b.cross(self.w).normalize();
            self.v = self.w.cross(self.u);

            self.half_fov_y = self.half_fov_y / 2.0;

            //let fovy_rads = self.fov_y * std::f32::consts::PI / 180.0;
            
            println!("2");
            // Calc vertical fov
            let aspect_ratio = self.width / self.heigh;
            println!("2.0");
            let tan_of_fovy = tan(self.fov_y);

            println!("Tan of fovy");
            self.fov_x = 2.0 * arctan(tan_of_fovy * aspect_ratio);

            println!("2.1");
            self.tan_half_fov_x = tan(self.fov_y / 2.0);
            println!("2.2");
            self.tan_half_fov_y = tan(self.fov_x);

            println!("3");
            self.half_height = self.heigh / 2.0;
            self.half_width = self.width / 2.0;

        }

        pub fn from(look_from: Vector3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, fov_y: f32) -> Self {
            let camera = Camera{
                width: 400.0,
                heigh: 300.0,
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
                w: Vector3{x: 0.0, y: 0.0, z: 0.0},
                u: Vector3{x: 0.0, y: 0.0, z: 0.0},
                v: Vector3{x: 0.0, y: 0.0, z: 0.0}
            };
            return camera;
        }
    }
}

