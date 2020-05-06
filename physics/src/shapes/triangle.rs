use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;



// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
pub struct Triangle
{
    pub a: Point<f32>,
    pub b: Point<f32>,
    pub c: Point<f32>
}

impl Triangle{
    /// Creates a Triangle
    pub fn new(a: Point<f32>, b: Point<f32>, c: Point<f32>) -> Triangle{
        return Triangle{a: a, b: b, c: c};
    }

    /// Creates and returns a RigidBody corresponding to the 'Triangle' type
    pub fn process_triangle(triangle: Triangle) -> ShapeHandle<f32>{
        // Points of the Triangle
        let a = triangle.a;
        let b = triangle.b;
        let c = triangle.c;

        // Creation of a Triangle we'll need later to make a RigidBody and Collider
        let tri = ShapeHandle::new(shape::Triangle::new(a, b, c));

        return tri;
    }
}
