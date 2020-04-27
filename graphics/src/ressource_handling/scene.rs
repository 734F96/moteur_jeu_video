use super::Object;
use crate::engine::{Graphical, Frame, Camera};
use crate::misc::{Similarity, new_vertexbuffer};


/**
A scene contains pointers to existing ressources and datas to place them in the space.
*/
pub struct Scene {
    pub objects: Vec<(Vec<Object>, Vec<Similarity>)>,
    pub camera: Camera
}


impl Scene {
    /// creates a scene
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            camera: Camera::new(2.0)
        }
    }

    /// Adds some objects to the scene
    pub fn add(&mut self, meshes: Vec<Object>, instances: Vec<Similarity>) {
        self.objects.push((meshes, instances));
    }

    pub fn update_aspect_ratio(&mut self, gr: &Graphical)
    {
        self.camera.update_aspect_ratio(gr);
    }
    
    /// Makes the graphic engine renders the scene. (maybe a bad idea)
    pub fn render(&mut self, gr: &Graphical, frame: &mut Frame)
    {
        self.camera.update_aspect_ratio(gr);
        self.objects.iter().for_each(|(objects, instances)| {
            let vbo = new_vertexbuffer(&gr.display, instances);
            objects
                .iter()
                .for_each(|ob|
                          frame.draw(&gr, &ob, &vbo, &self.camera)
                )
        });
        
    }
    
}
