use super::{Object, Light, Lights, N_MAX_LIGHTS};
use crate::engine::{Graphical, Frame, Camera, Display};
use crate::misc::{Similarity, new_vertexbuffer};

use glium::uniforms::UniformBuffer;

/**
A scene contains pointers to existing ressources and datas to place them in the space.
*/
pub struct Scene {
    pub objects: Vec<(Vec<Object>, Vec<Similarity>)>,
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
    pub fn add(&mut self, meshes: Vec<Object>, instances: Vec<Similarity>) {
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
    pub fn render(&mut self, gr: &Graphical, frame: &mut Frame)
    {

        //self.lights.print();

        
        self.camera.update_aspect_ratio(gr);
        self.objects.iter().for_each(|(objects, instances)| {
            let vbo = new_vertexbuffer(&gr.display, instances);
            objects
                .iter()
                .for_each(|ob|
                          frame.draw(&gr, &ob, &vbo, &self.camera, &self.lights)
                )
        });
    }
    
}
