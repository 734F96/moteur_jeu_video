use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;



// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
pub struct Segment
{
    pub a: Point<f32>,
    pub b: Point<f32>
}

impl Segment{
    /// Creates a Segment
    pub fn new(a: Point<f32>, b: Point<f32>) -> Segment{
        return Segment{a: a, b: b};
    }

    /// Creates and returns a RigidBody corresponding to the 'Segment' type
    pub fn process_segment(segment: Segment) -> ShapeHandle<f32>{
        // Points of the Segment
        let a = segment.a;
        let b = segment.b; 

        // Creation of a Segment we'll need later to make a RigidBody and Collider
        let seg = ShapeHandle::new(shape::Segment::new(a, b));

        return seg;
    }
}
