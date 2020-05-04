extern crate nalgebra as na;

use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::geometry::Point2;



// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
pub struct Polyline
{
    pub points: Vec<Point<f32>>,
    pub indices: Option<Vec<Point2<usize>>>
}

impl Polyline{
    /// Creates a Polyline
    pub fn new(points: Vec<Point<f32>>, indices: Option<Vec<Point2<usize>>>) -> Polyline{
        return Polyline{points: points, indices: indices};
    }

    /// Creates and returns a RigidBody corresponding to the 'Polyline' type
    pub fn process_polyline(polyline: Polyline) -> ShapeHandle<f32>{
        // Points and indices of the Polyline
        let points = polyline.points;
        let indices = polyline.indices;

        // Creation of a Polyline we'll need later to make a RigidBody and Collider
        let polyl = ShapeHandle::new(shape::Polyline::new(points, indices));

        return polyl;
    }
}
