pub mod shapes {
    use crate::camera::camera::Ray;
    use crate::ray_tracer::ray_tracer::HitInfo;
    use cgmath::num_traits::pow;
    use cgmath::Matrix4;
    use cgmath::Vector3;
    use cgmath::Vector4;
    use cgmath::{
        EuclideanSpace, InnerSpace, Matrix, Matrix3, Point3, SquareMatrix, Transform, Zero,
    };
    use rust_math::num::sqrt;

    const M_PI: f32 = std::f32::consts::PI;

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
        fn pre_calc(&mut self);
        fn intersection(&self, ray: &Ray) -> HitInfo;
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
        radius: f32,
        g_shape: GeometricShape,
    }

    impl Sphere {
        pub fn from(x: f32, y: f32, z: f32) -> Sphere {
            let mut sphere = Sphere::default();
            sphere.x = x;
            sphere.y = y;
            sphere.z = z;
            return sphere;
        }

        pub fn default() -> Sphere {
            Sphere {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                radius: 0.0,
                g_shape: GeometricShape {
                    size: 0.0,
                    material: Vector3 { x: 0, y: 0, z: 0 },
                    transform: Matrix4::zero(),
                    inverse_transform: Matrix4::zero(),
                    inverse_transpose_transform: Matrix4::zero(),
                    inverse_transpose_transform_3x3: Matrix3::zero(),
                },
            }
        }
    }

    impl AsGShape for Sphere {
        fn as_g_shape(&self) -> &GeometricShape {
            &self.g_shape
        }

        fn pre_calc(&mut self) {
            // inverseTransform = glm::inverse(transform);
            self.g_shape.inverse_transform = self.g_shape.transform.invert().unwrap();
            // inverseTransposeTransform = glm::transpose(inverseTransform);
            self.g_shape.inverse_transpose_transform = self.g_shape.inverse_transform.transpose();
            // inverseTransposeTransform3x3 = mat3(inverseTransposeTransform);
            // self.g_shape.inverse_transpose_transform_3x3 = Matrix3::from(self.g_shape.inverse_transpose_transform.);
            self.g_shape.inverse_transpose_transform_3x3 = Matrix3::from_cols(
                self.g_shape.inverse_transpose_transform.row(0).truncate_n(3),
                self.g_shape.inverse_transpose_transform.row(1).truncate_n(3),
                self.g_shape.inverse_transpose_transform.row(0).truncate_n(3),
            );
        }

        fn intersection(&self, ray: &Ray) -> HitInfo {
            let old_p0 = ray.o;
            let old_p1 = ray.d;

            let mut inverse = self.g_shape.inverse_transform;

            // apply inverse transform to the ray
            let o = (inverse * Vector4::new(ray.o.x, ray.o.y, ray.o.z, 1.0)).truncate(); // 1 -> point
            let d = (inverse * Vector4::new(ray.d.x, ray.d.y, ray.d.z, 1.0)).truncate().normalize(); // 0 -> vector

            //A = Xd^2 + Yd^2 + Zd^2 = 1 since |P1|
            let a = pow(d.x, 2) + pow(d.y, 2) + pow(d.z, 2);

            // B = 2 * (Xd * (X0 - Xc) + Yd * (Y0 - Yc) + Zd * (Z0 - Zc))
            let b = 2.0 * (d.x * (o.x - self.x) + (d.y * (o.y - self.y)) + (d.z * (o.z - self.z)));

            // C = (X0 - Xc) ^ 2 + (Y0 - Yc) ^ 2 + (Z0 - Zc) ^ 2 - Sr ^ 2
            let c = pow((o.x - self.x), 2) + pow((o.y - self.y), 2) + pow((o.z - self.z), 2)
                - pow(self.radius, 2);

            let discriminant = (b * b) - (4.0 * a * c);

            if discriminant < 0.0
            // No intersection
            {
                return HitInfo::no_hit();
            } else if discriminant >= 0.0 {
                let sqrt_discriminant = sqrt(discriminant);

                // calc t0 and check if it is valid
                let t0 = (-b - sqrt_discriminant) / (2.0 * a);
                let mut t1 = (-b + sqrt_discriminant) / (2.0 * a);

                let mut t_value = 0.0;

                // t0 is valid
                if t0 > 0.0 {
                    t_value = t0;
                } else {
                    // then it is t1
                    t1 = (-b + sqrt_discriminant) / (2.0 * a);
                    t_value = t1;

                    if t1 <= 0.0 {
                        return HitInfo::no_hit();
                    };
                }

                // compute intersection
                // take intersection point back to the actual object's transform
                let temp = (o + d * t_value);
                let intersection_p = Vector4::new(temp.x, temp.y, temp.z, 1.0);

                let intersection_obj_space = self.g_shape.transform * intersection_p;

                // Calc normal
                let normal =
                    (intersection_p - Vector4::new(self.x, self.y, self.z, 0.0)).normalize();
                let normal_transformed =
                    (self.g_shape.inverse_transpose_transform_3x3 * normal.truncate()).normalize();

                // Calc depth value
                let t = (intersection_obj_space.truncate() - old_p0).magnitude();

                // Set output
                return HitInfo::from(
                    true,
                    t,
                    intersection_obj_space.truncate(),
                    normal_transformed,
                    ray.clone(),
                );
            }

            return HitInfo::no_hit();
        }
    }
}
