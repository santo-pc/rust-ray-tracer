#[cfg(test)]
mod test {

    use cgmath::{Matrix4, One, Vector3, Zero};

    use crate::camera::camera::Ray;
    use crate::ray_tracer::ray_tracer::TestHit;
    use crate::shapes::shapes::AsGShape;
    use crate::Sphere;

    #[test]
    fn should_intersect_sphere() {
        // given a sphere at 10, 0, 0 and a ray with direction 1,0,0
        //
        let transform = Matrix4::one(); // from_translation(Vector3 { x: 10.0, y: 0.0, z: 0.0 });
        println!["Transform: {:?}", transform];
        let sphere = Sphere::from(20.0, 0.0, 0.0, 1.0, transform);

        // when calculating the intersection
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 10000.0);
        let hit = sphere.intersection(&ray);
        println!["Hit result: {:?}", hit];

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
        println!["Transform: {:?}", transform];
        let sphere = Sphere::from(100.0, 100.0, 100.0, 1.0, transform);

        // when calculating the intersection
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 100000.0);
        let hit = sphere.intersection(&ray);
        println!["Hit result: {:?}", hit];

        // it should not hit
        match hit {
            TestHit::NoHit => println!("No Hit"),
            TestHit::Hit(_) => panic!("Should not hit"),
        }
    }
}
