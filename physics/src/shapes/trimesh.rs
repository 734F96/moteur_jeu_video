extern crate nalgebra as na;

use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::geometry::{Point2, Point3};



// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
pub struct TriMesh
{
    pub points: Vec<Point<f32>>,
    pub indices: Vec<Point3<usize>>,
    pub uvs: Option<Vec<Point2<f32>>>
}

impl TriMesh{
    /// Creates a TriMesh
    pub fn new(points: Vec<Point<f32>>, indices: Vec<Point3<usize>>, uvs: Option<Vec<Point2<f32>>>) -> TriMesh{
        return TriMesh{points: points, indices: indices, uvs: uvs};
    }

    /// Creates and returns a RigidBody corresponding to the 'TriMesh' type
    pub fn process_trimesh(trimesh: TriMesh) -> ShapeHandle<f32>{
        // Points, indices and uvs of the TriMesh
        let points = trimesh.points;
        let indices = trimesh.indices;
        let uvs = trimesh.uvs;

        // Creation of a TriMesh we'll need later to make a RigidBody and Collider
        let trim = ShapeHandle::new(shape::TriMesh::new(points, indices, uvs));

        return trim;
    }
}
