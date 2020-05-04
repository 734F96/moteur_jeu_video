extern crate nalgebra as na;

use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Debug, Clone)]
pub struct Cuboid
{
    pub vector: Vector3<f32>
}

impl Cuboid{
    /// Creates a Cuboid
    pub fn new(vector: Vector3<f32>) -> Cuboid{
        return Cuboid{vector: vector};
    }

    /// Creates and returns a RigidBody corresponding to the 'Cuboid' type
    pub fn process_cuboid(cuboid: Cuboid) -> ShapeHandle<f32>{
        // Vector of the Cuboid
        let vector = cuboid.vector;

        // Creation of a Cuboid we'll need later to make a RigidBody and Collider
        let cub = ShapeHandle::new(shape::Cuboid::new(vector));

        return cub;
    }
}
