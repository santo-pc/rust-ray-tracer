
pub mod camera {    
    use cgmath::Vector3; 
    use cgmath::prelude::*;
    use rust_math::trigonometry::*; 
    const M_PI: f32 = 3.14159265358979323846;

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

    #[derive(Debug)]
    pub struct Ray {
        pub o: Vector3<f32>,
        pub d: Vector3<f32>,
        pub t: f32

    }

    impl Ray {          
        pub fn new(o: Vector3<f32>, d: Vector3<f32>, t: f32) -> Ray {
           Ray{o, d, t} 
        }
    }

    impl Camera {
        pub fn pre_calc(&mut self) {
             // calc coordframe
            let a = self.look_from - self.look_at;
            let b = self.up;
            
            self.w = a.normalize();
            self.u = b.cross(self.w).normalize();
            self.v = self.w.cross(self.u);

            self.half_fov_y = self.fov_y / 2.0;

            // Calc vertical fov
            let fovy_rads = self.fov_y * M_PI / 180.0;  
            let aspect_ratio = self.width / self.heigh;

            self.fov_x = 2.0 * arctan(tan(fovy_rads/ 2.0) * aspect_ratio);

            // pre cal tan of half fovs
            self.tan_half_fov_x = tan(self.fov_x / 2.0);
            self.tan_half_fov_y = tan(fovy_rads / 2.0);

            self.half_height = self.heigh / 2.0;
            self.half_width = self.width / 2.0;

        }

        pub fn from(look_from: Vector3<f32>, look_at: Vector3<f32>, up: Vector3<f32>, fov_y: f32) -> Self {
           Camera{
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
            }
        }

        pub fn ray_thru_pixel(self, x: f32, y: f32) -> Ray {
            let alpha = self.tan_half_fov_x * ((x - (self.half_width)) / self.half_width);
            let beta = self.tan_half_fov_y * ((self.half_height - y)/ self.half_height);

            let o = self.look_from;
            let d = ((alpha * self.u) + (-beta * self.v) - self.w).normalize();

            let t = 10000.0;

            return Ray::new(o, d, t);

        }

        pub fn eye_ray(self, x: i32, y: i32, img_width: i32, img_height: i32) -> Ray {
        
            let w_dir = self.w;
            let u_dir = self.u;
            let v_dir = self.v;
	    
            let aspect_ratio: f32 = img_width as f32 / img_height as f32;

            let top = tan(self.fov_y / 2.0);
            let right = aspect_ratio * top;
	        let bottom = -top;
	        let left = -right;

            // transform x and y into camera space
            let im_plane_u_pos = left + (right - left) * (x as f32 / img_width as f32);
            let im_plane_v_pos = bottom + (top - bottom) * (y as f32 / img_height as f32);

            return Ray::new(self.look_from, ((im_plane_u_pos*u_dir) + im_plane_v_pos*v_dir - w_dir).normalize(), 99999.0);


        } 
    }
}

