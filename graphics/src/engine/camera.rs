use nalgebra_glm::{look_at, rotate_x_vec3, rotate_y_vec3, rotate_z_vec3};
use nalgebra::{Matrix3, Vector3, Perspective3};

use super::Graphical;

pub struct Projection
{
  zfar : f32,
  znear :f32
    
}

/**
A simple camera
*/

pub struct Camera {
    position: Vector3<f32>,
    up: Vector3<f32>,
    orientation: Vector3<f32>,
    aspect_ratio: f32,
    fov :f32,
    projection : Projection
}

impl Camera {
    /** Constructor of Frame  */
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Vector3::new(0., 0., 0.), 
            orientation: Vector3::new(0., 0., -1.), 
            up: Vector3::new(0., 1., 0.),
            aspect_ratio: aspect_ratio, // Ratio for the printing on the screen
	    fov:3.141592 / 3.0,
	    projection: Projection{zfar:1024.0,znear : 0.1}
 	
        }
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width/height ; 
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = Vector3::new(position[0], position[1], position[2]);
    }

    /// Displace the camera
    pub fn relative_move(&mut self, displacement: Vector3<f32>) {
        let side = self.orientation.cross(&self.up);

	// Create the transfer_matrix from the base "Scene" to the base "Camera"
	let transfer_matrix = Matrix3::from_columns(&[self.orientation, self.up, side]);

	// Calculate the new position of the camera in the base "Scene"
	let position_vec = transfer_matrix*displacement;
	self.position = Vector3::new(position_vec[0]+self.position[0], position_vec[1]+self.position[1], position_vec[2]+self.position[2]);
    }


    pub fn set_direction(&mut self, orientation: Vector3<f32>) {
        self.orientation = orientation;
    }

    /// Rotation of the Camera around its axises
    pub fn rotation_relative(&mut self, rotation: Vector3<f32>) {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

        self.orientation = rotate_x_vec3(&self.orientation, rotation[0]) ;
	self.orientation = rotate_y_vec3(&self.orientation, rotation[1]) ;
	self.orientation = rotate_z_vec3(&self.orientation, rotation[2]).normalize() ;

        self.up = rotate_x_vec3(&self.up, rotation[0]) ;
	self.up = rotate_y_vec3(&self.up, rotation[1]) ;
	self.up = rotate_z_vec3(&self.up, rotation[2]) ;

    }

    /// Rotation of the Camera around axis x y and z of the Scene
    pub fn rotation(&mut self, rotation: Vector3<f32>) {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

	let orientation_scene = Vector3::new(0., 0., -1.);
	let up_scene = Vector3::new(0., 1., 0.);

	self.orientation = rotate_x_vec3(&orientation_scene, rotation[0]);
	self.orientation = rotate_y_vec3(&self.orientation, rotation[1]);
	self.orientation = rotate_z_vec3(&self.orientation, rotation[2]).normalize();

	self.up = rotate_x_vec3(&up_scene, rotation[0]);
	self.up = rotate_y_vec3(&self.up, rotation[1]);
	self.up = rotate_z_vec3(&self.up, rotation[2]);

    }


    /** Move the Objects of the scene according to the rotation of the Camera, Return the matrix of the view */
    /**
    returns the view matrix of the camera
    It will be used by the shaders to displace the objects of the scene to put them at the right place, with the right orientation and size.
     */
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {

	let center = Vector3::new(self.orientation[0] + self.position[0], self.orientation[1] + self.position[1], self.orientation[2] + self.position[2]);
	let look_at = look_at(&self.position, &center, &self.up);
	let view_matrix = look_at.as_ref();
	*view_matrix 

    }

    /// Return the matrix of the perspective
    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4] {
        //let (width, height) = target.get_dimensions();
	let perspective = Perspective3::new(self.aspect_ratio, self.fov, self.projection.znear, 		self.projection.zfar);
	let perspective_matrix = perspective.as_matrix().as_ref() ;
	*perspective_matrix
    }

    pub fn update_aspect_ratio(&mut self, gr: &Graphical)
    {
        let (w, h) = gr.display.display.get_framebuffer_dimensions();
        self.set_aspect_ratio(w as f32, h as f32);
    }
}
