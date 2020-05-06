extern crate nalgebra as na;

use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;
use na::base::DMatrix;



// We implement the Clone trait to the structure
#[derive(Clone, Debug)]
pub struct HeightField
{
    pub heights: DMatrix<f32>,
    pub scale: Vector3<f32>
}

impl HeightField{
    /// Creates a HeightField
    pub fn new(heights: DMatrix<f32>, scale: Vector3<f32>) -> HeightField{
        return HeightField{heights: heights, scale: scale};
    }

    /// Creates and returns a RigidBody corresponding to the 'HeightField' type
    pub fn process_heightfield(heightfield: HeightField) -> ShapeHandle<f32>{
        // Height and scale of the HeightField
        let heights = heightfield.heights;
        let scale = heightfield.scale;

        // Creation of a HeightField we'll need later to make a RigidBody and Collider
        let heightf = ShapeHandle::new(shape::HeightField::new(heights, scale));

        return heightf;
    }
}
