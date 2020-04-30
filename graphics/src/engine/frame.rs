use glium::{Surface, uniform};

use super::{Graphical, Params, Camera};
use crate::ressource_handling::{Object, Material, Lights};
use crate::misc::Similarity;



/**
Where the scene is being constructed.
Destroyed uppon displaying.
This is a wrapper around glium::Frame with custom methods.
*/
pub struct Frame {
    pub frame: glium::Frame,
}

impl Frame {
    /// Constructor of Frame
    pub fn new(gr: &Graphical) -> Self {
        Self {
            frame: gr.display.display.draw(),
        }
    }


    /**
    Draws several instances of an Object in the Frame using the similarities contained by the VBO per_instance.
    Calls fn draw_group for each group of Object.
     */
    pub fn draw(
        &mut self,
        gr: &Graphical,
        obj: &Object,
        per_instance: &glium::VertexBuffer<Similarity>, // position
        camera: &Camera,
        lights: &Lights
    ) {
        obj.data
            .iter()
            .for_each(|(group, program)|
                      self.draw_group
                      (
                          &group.vertexes,
                          per_instance,
                          &group.material,
                          gr.program.get(*program)
                              .unwrap(),
                          &obj.params,
                          camera,
                          lights
                      )
            );
    }
    
    /// Draws a group of Object (part of the Object) in the Frame, called by fn draw, with a specific material and program
    pub fn draw_group(
        &mut self,
        vertex_buffer: &glium::vertex::VertexBufferAny,
        per_instance: &glium::VertexBuffer<Similarity>,
        material: &Material,
	program: &glium::Program,
        params: &Params,
        camera: &Camera,
        lights: &Lights
    ) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        match material {
            Material::Textured {
                texture,
                specular_color,
                specular_exponent,
                opacity,
            } => {
                /*
                This is a bit ugly but necessary.
                Creates the mipmap of the textures and also binds it.
                We look forward to replace this by something cleaner (or safer).
                 */
		unsafe {texture.generate_mipmaps();}
                
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        program,
                        &uniform! {
                            texture: texture,
                            view_matrix: camera.get_view_matrix(),
                            perspective_matrix: camera.get_perspective_matrix(),

                            specular_color: *specular_color,
                            specular_exponent: *specular_exponent,
                            opacity: *opacity,
                            

                            lights_type: &lights.light_type,
                            lights_intensity: &lights.intensity,
                            lights_pos: &lights.position,
                            lights_dir: &lights.direction,
                            lights_col: &lights.colour,

                        },
                        &params.parameters,
                    )
                    .unwrap();
            }
	    
            Material::NonTextured {
                ambiant_color,
                diffuse_color,
                specular_color,
                specular_exponent,
                emission_color,
                opacity,
            } => {
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        program,
                        &uniform! {view_matrix: camera.get_view_matrix(),
                                   perspective_matrix: camera.get_perspective_matrix(),
                                   ambiant: *ambiant_color,
                                   diffuse: *diffuse_color,
                                   specular: *specular_color,
                                   specular_exponent: *specular_exponent,
                                   emission: *emission_color,
                                   opacity: *opacity,


                                   lights_type: &lights.light_type,
                                   lights_intensity: &lights.intensity,
                                   lights_pos: &lights.position,
                                   lights_dir: &lights.direction,
                                   lights_col: &lights.colour,
},
                        &params.parameters,
                    )
                    .unwrap();
            }
            _ => {
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        program,
                        &uniform! {view_matrix: camera.get_view_matrix() },
                        &params.parameters,
                    )
                    .unwrap();
            }
        }
    }

    
    /// Clear and resets the Frame in colour and depth.
    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    }

    /**
    Sends the Frame to the Graphical Card.
    The frame then cannot be used anymore
     */
    pub fn swap(self) {
        self.frame.finish().unwrap();
    }
}
