
pub mod shapes {
    use::cgmath::Vector3;
    use::cgmath::Vector4;
    use::cgmath::Matrix4;

    #[derive(Debug)]
    pub struct HitInfo {
        has_intersected: bool,
        t_value: f32,
        p: Vector3<f32>,
        n: Vector3<f32>
    }

    impl HitInfo {
        pub fn default() -> HitInfo {
            HitInfo{
                has_intersected: false,
                t_value: 0.0,
                p: Vector3::new(0.0, 0.0, 0.0),
                n: Vector3::new(0.0, 0.0, 0.0)
            }
        }
    }


    #[derive(Debug)]
    pub struct GeometricShape {
        //type needed?
        size: f32,
        material: Vector3<i32>,
        transform: Matrix4<f32>,
        inverse_transform: Matrix4<f32>,
        inverse_transpose_transform: Matrix4<f32>,
        inverse_transpose_transform_3x3: Matrix4<f32>,
        
    }

    pub trait AsGShape {
        fn as_g_shape(&self) -> &GeometricShape;
        fn pre_calc(&self); 
        fn intersection(&self) -> bool; 
    }
    
    impl GeometricShape {
        pub fn intersection() -> bool {
            return true;
        }
    }
    
    #[derive(Debug)]
    pub struct Sphere {
        x: f32,
        y: f32,
        z: f32,
        radious: f32,
        g_shape: GeometricShape
    }

    impl Sphere {
        pub fn default() -> Sphere {
            Sphere {
                x: 0.0, y: 0.0, z: 0.0, radious: 0.0, 
                g_shape: GeometricShape { 
                    size: 0.0, 
                    material: Vector3 { x: 0, y: 0, z: 0 }, 
                    transform: Matrix4 { 
                        x: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                        y: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, 
                        z: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0}, 
                        w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0} 
                    }, 
                    inverse_transform: Matrix4 { 
                        x: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                        y: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, 
                        z: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0}, 
                        w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0} 
                    },                     
                    inverse_transpose_transform: Matrix4 { 
                        x: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                        y: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, 
                        z: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0}, 
                        w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0} 
                    },                     
                    inverse_transpose_transform_3x3: Matrix4 { 
                        x: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                        y: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, 
                        z: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0}, 
                        w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0} 
                    }                 
                } 
            }
        }
    }

    impl AsGShape for Sphere {
        fn as_g_shape(&self) -> &GeometricShape {
            &self.g_shape
        }
        
        fn pre_calc(&self) {
            unimplemented!();
            //self.g_shape.inverse_transform = self.g_shape.transform.inverse(); 
//            self.g_shape.inverse_transpose_transform = self.g_shape.inverse_transform.transpose();
  //          self.g_shape.inverse_transpose_transform_3x3 = self.g_shape.inverse_transform.transpose();
        }

        fn intersection(&self) -> bool {
            false
        }
    }
}
