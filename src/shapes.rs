#[cfg(test)]
mod test;

pub mod shape_components {
    use crate::{
        camera::camera_view::Ray,
        ray_tracer::tracer::{Color, HitInfo, TestHit},
    };
    use cgmath::{num_traits::pow, InnerSpace, Matrix, Matrix4, SquareMatrix};
    use cgmath::{Matrix3, One, Zero};
    use cgmath::{Vector3, Vector4};

    #[derive(Debug)]
    pub struct GeometricShape {
        //type needed?
        _size: f64,
        _material: Vector3<i32>,
        transform: Matrix4<f64>,
        inverse_transform: Matrix4<f64>,
        // inverse_transpose_transform: Matrix4<f64>,
        inverse_transpose_transform_3x3: Matrix3<f64>,
    }

    pub trait AsGShape {
        fn as_g_shape(&self) -> &GeometricShape;
        // fn pre_calc(&mut self);
        fn intersection(&self, ray: &Ray) -> TestHit;
    }

    impl std::default::Default for GeometricShape {
        fn default() -> Self {
            Self {
                _size: 0.0,
                _material: Vector3::zero(),
                transform: Matrix4::one(),
                inverse_transform: Matrix4::one(),
                // inverse_transpose_transform: Matrix4::one(),
                inverse_transpose_transform_3x3: Matrix3::one(),
            }
        }
    }
    impl GeometricShape {
        pub fn intersection() -> bool {
            true
        }

        pub fn from(transform: Matrix4<f64>) -> GeometricShape {
            println!("Transform is {:?} ", transform);

            let inverse_transform = transform.invert().unwrap();
            let inverse_transpose_transform = inverse_transform.transpose();
            let inverse_transpose_transform_3x3 = Matrix3::new(
                inverse_transpose_transform.x.x,
                inverse_transpose_transform.x.y,
                inverse_transpose_transform.x.z,
                inverse_transpose_transform.y.x,
                inverse_transpose_transform.y.y,
                inverse_transpose_transform.y.z,
                inverse_transpose_transform.z.x,
                inverse_transpose_transform.z.y,
                inverse_transpose_transform.z.z,
            );

            GeometricShape {
                transform,
                inverse_transform,
                // inverse_transpose_transform,
                inverse_transpose_transform_3x3,
                ..Default::default()
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Sphere {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub radius: f64,
        pub g_shape: GeometricShape,
    }

    impl Sphere {
        pub fn from(x: f64, y: f64, z: f64, radius: f64, transform: Matrix4<f64>) -> Sphere {
            Sphere { x, y, z, radius, g_shape: GeometricShape::from(transform) }
        }
    }

    impl AsGShape for Sphere {
        fn as_g_shape(&self) -> &GeometricShape {
            &self.g_shape
        }

        fn intersection(&self, ray: &Ray) -> TestHit {
            // apply inverse transform to the ray
            let o = (self.g_shape.inverse_transform * Vector4::new(ray.o.x, ray.o.y, ray.o.z, 1.0))
                .truncate(); // 1 -> point

            let d = (self.g_shape.inverse_transform * Vector4::new(ray.d.x, ray.d.y, ray.d.z, 0.0))
                .truncate()
                .normalize(); // 0 -> vector

            //A = Xd^2 + Yd^2 + Zd^2 = 1 since |P1|
            let a = pow(d.x, 2) + pow(d.y, 2) + pow(d.z, 2);

            // B = 2 * (Xd * (X0 - Xc) + Yd * (Y0 - Yc) + Zd * (Z0 - Zc))
            let b = 2.0 * (d.x * (o.x - self.x) + (d.y * (o.y - self.y)) + (d.z * (o.z - self.z)));

            // C = (X0 - Xc) ^ 2 + (Y0 - Yc) ^ 2 + (Z0 - Zc) ^ 2 - Sr ^ 2
            let c = pow(o.x - self.x, 2) + pow(o.y - self.y, 2) + pow(o.z - self.z, 2)
                - pow(self.radius, 2);

            let discriminant = (b * b) - (4.0 * a * c);

            // No intersection
            if discriminant < 0.0 {
                return TestHit::NoHit;
            }

            let sqrt_discriminant = f64::sqrt(discriminant);

            // calc t0 and check if it is valid
            let t0 = (-b - sqrt_discriminant) / (2.0 * a);

            let t_value: f64;

            // t0 is valid
            if t0 > 0.0 {
                t_value = t0;
            } else {
                // then it is t1
                t_value = (-b + sqrt_discriminant) / (2.0 * a);

                if t_value <= 0.0 {
                    return TestHit::NoHit;
                };
            }

            // compute intersection
            // take intersection point back to the actual object's transform
            let temp = o + d * t_value;
            let intersection_p = Vector4::new(temp.x, temp.y, temp.z, 1.0);

            let intersection_obj_space = self.g_shape.transform * intersection_p;

            // Calc normal
            let normal = (intersection_p - Vector4::new(self.x, self.y, self.z, 0.0)).normalize();
            let normal_transformed =
                (self.g_shape.inverse_transpose_transform_3x3 * normal.truncate()).normalize();

            // Calc depth value
            let t = (intersection_obj_space.truncate() - ray.o).magnitude();

            // Set output
            TestHit::Hit(HitInfo::from(
                t,
                intersection_obj_space.truncate(),
                normal_transformed,
                *ray,
                Color { r: 255, g: 0, b: 0 },
            ))
        }
    }

    #[derive(Debug)]
    pub struct Triangle {
        pub vertices: Vec<usize>,
        pub a: Vector4<f64>,
        pub b: Vector4<f64>,
        pub c: Vector4<f64>,
        pub g_shape: GeometricShape,
        a_transformed: Vector4<f64>,
        b_transformed: Vector4<f64>,
        c_transformed: Vector4<f64>,
    }

    impl Triangle {
        pub fn new(
            vertices: Vec<usize>,
            a: Vector4<f64>,
            b: Vector4<f64>,
            c: Vector4<f64>,
        ) -> Triangle {
            let mut triangle = Triangle {
                vertices,
                a,
                b,
                c,
                g_shape: GeometricShape::default(),
                a_transformed: Vector4::zero(),
                b_transformed: Vector4::zero(),
                c_transformed: Vector4::zero(),
            };
            triangle.pre_calc();
            triangle
        }

        fn pre_calc(&mut self) {
            self.g_shape.inverse_transform = self.g_shape.transform.invert().unwrap();
            self.g_shape.inverse_transform = self.g_shape.inverse_transform.transpose();
            self.a_transformed = self.g_shape.transform * self.a;
            self.b_transformed = self.g_shape.transform * self.b;
            self.c_transformed = self.g_shape.transform * self.c;
        }
    }

    impl AsGShape for Triangle {
        fn as_g_shape(&self) -> &GeometricShape {
            &self.g_shape
        }

        fn intersection(&self, ray: &Ray) -> TestHit {
            let a = self.a_transformed.truncate();
            let b = self.b_transformed.truncate();
            let c = self.c_transformed.truncate();
            let t_min: f64 = 0.0;
            let t_max: f64 = f64::MAX;

            let v_ab = b - a;
            let v_ac = c - a;
            let norm = v_ab.cross(v_ac).normalize();

            // Triangle plane intersection
            let output_t_value = norm.dot(a - ray.o) / norm.dot(ray.d);
            let q = ray.o + output_t_value * ray.d;

            // Compute barycentric coordinates
            let v_aq = q - a;
            let dab_ab = v_ab.dot(v_ab);
            let dab_ac = v_ab.dot(v_ac);
            let dac_ac = v_ac.dot(v_ac);
            let daq_ab = v_aq.dot(v_ab);
            let daq_ac = v_aq.dot(v_ac);

            // determinant
            let d = dab_ab * dac_ac - dab_ac * dab_ac;

            if d == 0.0 {
                return TestHit::NoHit;
            }

            let beta = (dac_ac * daq_ab - dab_ac * daq_ac) / d;
            let gamma = (dab_ab * daq_ac - dab_ac * daq_ab) / d;

            if (beta < 0.0 || gamma < 0.0)
                || (beta > 1.0 || gamma > 1.0)
                || (beta + gamma > 1.0)
                || (output_t_value < t_min || output_t_value > t_max)
            {
                return TestHit::NoHit;
            }

            // its a hit
            TestHit::Hit(HitInfo::from(
                output_t_value,
                q,
                norm,
                *ray,
                Color { r: 0, g: 255, b: 255 },
            ))
        }
    }
}
