use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;



// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
pub struct ConvexHull
{
    pub points: Vec<Point<f32>>
}

impl ConvexHull{
    /// Creates a ConvexHull
    pub fn new(points: Vec<Point<f32>>) -> ConvexHull{
        return ConvexHull{points: points};
    }

    /// Creates and returns a RigidBody corresponding to the 'ConvexHull' type
    pub fn process_convexhull(convexhull: ConvexHull) -> ShapeHandle<f32>{
        // Points of the ConvexHull
        let points = convexhull.points;

        // Creation of a ConvexHull we'll need later to make a RigidBody and Collider
        let convexh = ShapeHandle::new(shape::ConvexHull::try_from_points(&points).unwrap());

        return convexh;
    }
}
