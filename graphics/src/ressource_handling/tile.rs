use std::path::PathBuf;
use glium::
{
    texture::{RawImage2d, Texture2d},
    vertex::{VertexBuffer, VertexBufferAny},
};
use std::sync::Arc;

use super::{Material, Vertex};
use crate::engine::Display;
use base::{Base, EngineError};

/**
 A tile is a 2D rectangle with an image on it.
Stores the proportions of the image and the image itselve.
*/
#[derive(Debug, Clone)]
pub struct Tile
{
    pub texture: Arc<Material>,
    pub dims: Arc<VertexBufferAny>
}


impl Tile
{
    /// Creates a new Tile from the given image path
    pub fn new(path: PathBuf, display: &Display) -> Result<Self, EngineError>
    {
//        let image = base.open_image(path)?
//	    .to_rgba();

	let image = image::open(path)?.to_rgba();
	
        
        let (x, y) = image.dimensions();
        let max = x.max(y) as f32;
        let dims = ((x as f32)/max, (y as f32)/max);
        let image =
            RawImage2d::from_raw_rgba_reversed(
                &image.into_raw(),
                (x, y)
            );
        
        let texture = Texture2d::new(
            &display.display,
            image
        )?;
        let mat = Material::Textured
        {
            texture: texture,
            specular_color: [0.; 3],
            specular_exponent: 0.,
            opacity: 0.
        };

	let (w, h) = dims;

	let z = 0.;
	/*
	let mesh = vec![
	    Vertex{position: [0., 0., z], texture: [0., 0.], .. Default::default()},
	    Vertex{position: [w, 0., z], texture: [1., 0.], .. Default::default()},
	    Vertex{position: [w, h, z], texture: [1., 1.], .. Default::default()},
	    Vertex{position: [0., 0., z], texture: [0., 0.], .. Default::default()},
	    Vertex{position: [w, h, z], texture: [1., 1.], .. Default::default()},
	    Vertex{position: [0., h, z], texture: [0., 1.], .. Default::default()},
	];
*/
	let mesh = vec![
	    Vertex{position: [-w/2., -h/2., z], texture: [0., 0.], .. Default::default()},
	    Vertex{position: [w/2., -h/2., z], texture: [1., 0.], .. Default::default()},
	    Vertex{position: [w/2., h/2., z], texture: [1., 1.], .. Default::default()},
	    Vertex{position: [-w/2., -h/2., z], texture: [0., 0.], .. Default::default()},
	    Vertex{position: [w/2., h/2., z], texture: [1., 1.], .. Default::default()},
	    Vertex{position: [-w/2., h/2., z], texture: [0., 1.], .. Default::default()},
	];

        let vbo: VertexBufferAny = VertexBuffer::new(&display.display, &mesh).unwrap().into();


	
        Ok(
            Self
            {
                texture: Arc::new(mat),
                dims: Arc::new(vbo)
            }

        )

    }
}
