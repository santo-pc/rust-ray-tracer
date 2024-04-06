#[cfg(test)]
mod shapes_test {

    use std::f64::consts::PI;

    use cgmath::{Matrix4, One, Vector3};

    use crate::camera::camera_view::Ray;
    use crate::ray_tracer::tracer::TestHit;
    use crate::shapes::shape_components::AsGShape;
    use crate::Sphere;
    use approx::assert_relative_eq;

    #[test]
    fn should_intersect_sphere() {
        // given a sphere at 10, 0, 0 and a ray with direction 1,0,0
        let transform = Matrix4::one(); // from_translation(Vector3 { x: 10.0, y: 0.0, z: 0.0 });
        let sphere = Sphere::from(20.0, 0.0, 0.0, 1.0, transform);
        // when calculating the intersection
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 10000.0);
        let hit = sphere.intersection(&ray);

        // it should hit
        match hit {
            TestHit::NoHit => panic!("Should hit"),
            TestHit::Hit(_) => println!("Hit"),
        }
    }

    #[test]
    fn should_not_intersect_sphere() {
        // given a sphere at 10, 10, 0 and a ray with direction 1,0,0
        let transform = Matrix4::one(); // from_translation(Vector3 { x: 10.0, y: 0.0, z: 0.0 });
        let sphere = Sphere::from(100.0, 100.0, 100.0, 1.0, transform);

        // when calculating the intersection
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 100000.0);
        let hit = sphere.intersection(&ray);

        // it should not hit
        match hit {
            TestHit::NoHit => println!("No Hit"),
            TestHit::Hit(_) => panic!("Should not hit"),
        }
    }

    #[test]
    fn test_trig_funcs() {
        // given
        assert_eq!((PI / 2.0).sin(), 1.0);
        assert_eq!(0.0f64.sin(), 0.0);
        assert_relative_eq!((PI / 6.0).sin(), 0.5, epsilon = f64::EPSILON);
        assert_relative_eq!(PI.sin(), 0.0, epsilon = f64::EPSILON);
        assert_relative_eq!(1.0, 1.0, epsilon = f64::EPSILON);
        assert_relative_eq!((PI / 6.0).sin(), 0.5, epsilon = f64::EPSILON);
    }
}
