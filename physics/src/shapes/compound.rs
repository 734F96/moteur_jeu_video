use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Isometry;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Compound
{
    pub shapes: Vec<(Isometry<f32>, ShapeHandle<f32>)>
}

use std::fmt;
impl fmt::Debug for Compound
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Compound")
    }
}

impl Compound{
    /// Creates a Compound
    pub fn new(shapes: Vec<(Isometry<f32>, ShapeHandle<f32>)>) -> Compound{
        return Compound{shapes: shapes};
    }

    /// Creates and returns a RigidBody corresponding to the 'Compound' type
    pub fn process_compound(compound: Compound) -> ShapeHandle<f32>{
        // Shapes of the Compound
        let shapes = compound.shapes; 

        // Creation of a Compound we'll need later to make a RigidBody and Collider
        let comp = ShapeHandle::new(shape::Compound::new(shapes));

        return comp;
    }
}
