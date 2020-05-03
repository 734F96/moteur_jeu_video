use glium::vertex::VertexBuffer;
use std::sync::Arc;
use super::{Material, Vertex};

/**
This structure represents a group of vertex and their associated material.
It doesn't own the date, rather it owns a shared pointer to the data.
 */
#[derive(Clone)]
pub struct Group {
    pub vertexes: Arc<VertexBuffer<Vertex>>,
    pub material: Arc<Material>,
}


use std::fmt;
impl fmt::Debug for Group
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Group")
    }
}
