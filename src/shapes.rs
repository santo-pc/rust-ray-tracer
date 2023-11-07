#[cfg(test)]
mod test;

pub mod shapes {
    use crate::camera::camera::Ray;
    use crate::ray_tracer::ray_tracer::HitInfo;
    use crate::ray_tracer::ray_tracer::TestHit;
    use cgmath::num_traits::pow;
    use cgmath::Matrix4;
    use cgmath::Vector3;
    use cgmath::Vector4;
    use cgmath::{
        EuclideanSpace, InnerSpace, Matrix, Matrix3, One, Point3, SquareMatrix, Transform, Zero,
    };
    use rust_math::num::sqrt;

    #[derive(Debug)]
    pub struct GeometricShape {
        //type needed?
        size: f32,
        material: Vector3<i32>,
        transform: Matrix4<f32>,
        inverse_transform: Matrix4<f32>,
        inverse_transpose_transform: Matrix4<f32>,
        inverse_transpose_transform_3x3: Matrix3<f32>,
    }

    pub trait AsGShape {
        fn as_g_shape(&self) -> &GeometricShape;
        // fn pre_calc(&mut self);
        fn intersection(&self, ray: &Ray) -> TestHit;
    }

    impl std::default::Default for GeometricShape {
        fn default() -> Self {
            Self {
                size: 0.0,
                material: Vector3::zero(),
                transform: Matrix4::one(),
                inverse_transform: Matrix4::one(),
                inverse_transpose_transform: Matrix4::one(),
                inverse_transpose_transform_3x3: Matrix3::one(),
            }
        }
    }
    impl GeometricShape {
        pub fn intersection() -> bool {
            return true;
        }

        pub fn from(transform: Matrix4<f32>) -> GeometricShape {
            // inverseTransform = glm::inverse(transform);
            let inverse_transform = transform.invert().unwrap();
            // inverseTransposeTransform = glm::transpose(inverseTransform);
            let inverse_transpose_transform = inverse_transform.transpose();
            // inverseTransposeTransform3x3 = mat3(inverseTransposeTransform);
            // self.g_shape.inverse_transpose_transform_3x3 = Matrix3::from(self.g_shape.inverse_transpose_transform.);
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
                inverse_transpose_transform,
                inverse_transpose_transform_3x3,
                ..Default::default()
            }
        }
    }

    #[derive(Debug)]
    pub struct Sphere {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub radius: f32,
        pub g_shape: GeometricShape,
    }

    impl Sphere {
        pub fn from(x: f32, y: f32, z: f32, radius: f32, transform: Matrix4<f32>) -> Sphere {
            Sphere { x, y, z, radius, g_shape: GeometricShape::from(transform) }
        }

        pub fn default() -> Sphere {
            Sphere { x: 0.0, y: 0.0, z: 0.0, radius: 0.0, g_shape: GeometricShape::default() }
        }
    }

    impl AsGShape for Sphere {
        fn as_g_shape(&self) -> &GeometricShape {
            &self.g_shape
        }

        // fn pre_calc(&mut self) {
        //     // inverseTransform = glm::inverse(transform);
        //     self.g_shape.inverse_transform = self.g_shape.transform.invert().unwrap();
        //     // inverseTransposeTransform = glm::transpose(inverseTransform);
        //     self.g_shape.inverse_transpose_transform = self.g_shape.inverse_transform.transpose();
        //     // inverseTransposeTransform3x3 = mat3(inverseTransposeTransform);
        //     // self.g_shape.inverse_transpose_transform_3x3 = Matrix3::from(self.g_shape.inverse_transpose_transform.);
        //     self.g_shape.inverse_transpose_transform_3x3 = Matrix3::from_cols(
        //         self.g_shape.inverse_transpose_transform.row(0).truncate_n(3),
        //         self.g_shape.inverse_transpose_transform.row(1).truncate_n(3),
        //         self.g_shape.inverse_transpose_transform.row(0).truncate_n(3),
        //     );
        // }
        //
        fn intersection(&self, ray: &Ray) -> TestHit {
            // let old_p0 = ray.o;
            // let old_p1 = ray.d;

            return TestHit::NoHit;
            let ray = ray.clone();

            // let inverse = self.g_shape.inverse_transform;

            println! {"Sphere intersection1"};
            // apply inverse transform to the ray
            let o = (self.g_shape.inverse_transform * Vector4::new(ray.o.x, ray.o.y, ray.o.z, 1.0))
                .truncate(); // 1 -> point

            let d = (self.g_shape.inverse_transform * Vector4::new(ray.d.x, ray.d.y, ray.d.z, 0.0))
                .truncate()
                .normalize(); // 0 -> vector

            //A = Xd^2 + Yd^2 + Zd^2 = 1 since |P1|
            let a = pow(d.x, 2) + pow(d.y, 2) + pow(d.z, 2);

            println! {"Sphere intersection2"};
            // B = 2 * (Xd * (X0 - Xc) + Yd * (Y0 - Yc) + Zd * (Z0 - Zc))
            let b = 2.0 * (d.x * (o.x - self.x) + (d.y * (o.y - self.y)) + (d.z * (o.z - self.z)));

            // C = (X0 - Xc) ^ 2 + (Y0 - Yc) ^ 2 + (Z0 - Zc) ^ 2 - Sr ^ 2
            let c = pow(o.x - self.x, 2) + pow(o.y - self.y, 2) + pow(o.z - self.z, 2)
                - pow(self.radius, 2);

            let discriminant = (b * b) - (4.0 * a * c);

            println! {"Sphere intersection3"};
            // No intersection
            if discriminant < 0.0 {
                println! {"Sphere intersection4"};
                return TestHit::NoHit;
            } else if discriminant >= 0.0 {
                let sqrt_discriminant = sqrt(discriminant);

                println! {"Sphere intersection5"};
                // calc t0 and check if it is valid
                let t0 = (-b - sqrt_discriminant) / (2.0 * a);
                // let mut t1 = 0.0; // (-b + sqrt_discriminant) / (2.0 * a);

                let t_value: f32;

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

                println! {"Sphere intersection6"};
                // compute intersection
                // take intersection point back to the actual object's transform
                let temp = o + d * t_value;
                let intersection_p = Vector4::new(temp.x, temp.y, temp.z, 1.0);

                let intersection_obj_space = self.g_shape.transform * intersection_p;

                println! {"Sphere intersection7"};
                // Calc normal
                let normal =
                    (intersection_p - Vector4::new(self.x, self.y, self.z, 0.0)).normalize();
                let normal_transformed =
                    (self.g_shape.inverse_transpose_transform_3x3 * normal.truncate()).normalize();

                // Calc depth value
                let t = (intersection_obj_space.truncate() - ray.o).magnitude();

                println! {"Sphere intersection8"};
                // Set output
                return TestHit::Hit(HitInfo::from(
                    t,
                    intersection_obj_space.truncate(),
                    normal_transformed,
                    ray.clone(),
                ));
            } else {
                println! {"Sphere intersection9"};
                return TestHit::NoHit;
            }
        }
    }
}
