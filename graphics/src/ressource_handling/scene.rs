use super::{Object, Light, Lights, Handle, RessourcesHolder};
use crate::engine::{Graphical, Frame, Camera, Display};
use crate::misc::{Similarity, new_vertexbuffer};


use std::time::{Duration, Instant};

/**
A scene contains pointers to existing ressources and datas to place them in the space.
*/
pub struct Scene {
    pub objects: Vec<(Vec<Handle<Object>>, Vec<Similarity>)>,
    pub lights: Lights,
    pub camera: Camera
}


impl Scene {
    /// creates a scene
    pub fn new(disp: &Display) -> Self
    {
        Self {
            objects: Vec::new(),
            lights: Lights::new(&disp.display),
            camera: Camera::new(2.0)
        }
    }

    /// Adds some objects to the scene
    pub fn add(&mut self, meshes: Vec<Handle<Object>>, instances: Vec<Similarity>) {
        self.objects.push((meshes, instances));
    }

    pub fn add_light(&mut self, light: Light)
    {
        self.lights.push(light);
    }
    
    pub fn update_aspect_ratio(&mut self, gr: &Graphical)
    {
        self.camera.update_aspect_ratio(gr);
    }
    
    /// Makes the graphic engine renders the scene. (maybe a bad idea)
    pub fn render(&mut self,
		  gr: &Graphical,
		  ressources: &RessourcesHolder,
		  frame: &mut Frame)
    {

        self.camera.update_aspect_ratio(gr);
        self.objects.iter().for_each(|(objects, instances)| {


	    // 10 % of time spent	    
            let vbo = new_vertexbuffer(&gr.display, instances);
	    
            objects
                .iter()
                .for_each(|obj_handle|
                          frame.draw(&gr,
				     ressources.get_by_handle(*obj_handle),
				     &vbo,
				     &self.camera,
				     &self.lights)
                );
		
        });
    }


}
