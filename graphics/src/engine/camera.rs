use nalgebra_glm::{look_at, rotate_x_vec3, rotate_y_vec3, rotate_z_vec3, vec3, normalize};
use nalgebra::{Matrix3, Vector3, Perspective3};

use super::Graphical;
use std::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};

#[derive(Debug, Clone, Copy)]
pub struct Projection
{
  zfar: f32,
  znear: f32
    
}

/**
A simple camera
*/

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub h_angular: f32, // theta
    pub v_angular: f32, // phi
    pub position: Vector3<f32>,
    pub up: Vector3<f32>,
    pub forward: Vector3<f32>,
    pub side: Vector3<f32>,
    aspect_ratio: f32,
    fov :f32,
    projection : Projection
}

impl Default for Camera
{
    fn default() -> Self {
	Self::new(1.)
    }
}

impl Camera {

    /** Constructor of Camera  */
    pub fn new(aspect_ratio: f32) -> Self
    {
        Self
	{
	    h_angular: 0.,
	    v_angular: 0.,
            position: Vector3::new(0., 0., 0.), 
            forward: Vector3::new(0., 0., -1.), 
            up: Vector3::new(0., 1., 0.),
            side: Vector3::new(1., 0., 0.),
            aspect_ratio: aspect_ratio, // Ratio for the printing on the screen
	    fov:3.141592 / 3.0,
	    projection: Projection{zfar:1024.0,znear : 0.1}
 	
        }
    }


    fn compute_forward(theta: f32, phi: f32) -> Vector3<f32>
    {
	let cosphi1 = 1. + phi.cos();
	normalize(
	    &vec3(theta.cos()*cosphi1,
		  phi.sin(),
		  theta.sin()*cosphi1)
	)
    }

    pub fn update_angles(&mut self)
    {
	let h_ang = self.h_angular;
	let v_ang = self.v_angular;
	self.forward = Self::compute_forward(h_ang, v_ang);
	self.up = Self::compute_forward(h_ang, v_ang + FRAC_PI_2);
        self.side = self.forward.cross(&self.up);
    }

    pub fn rotate(&mut self, h_ang: f32, v_ang: f32)
    {
	self.h_angular += h_ang;
	self.v_angular -= v_ang;

	if self.v_angular > FRAC_PI_2 - 0.01
	{
	    self.v_angular = FRAC_PI_2 - 0.01;
	}
	if self.v_angular < -PI + 0.01
	{
	    self.v_angular = -PI + 0.01;
	}

	self.update_angles();
    }
    
    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width/height ; 
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = Vector3::new(position[0], position[1], position[2]);
    }

    pub fn translate_x(&mut self, delta: f32){ self.position[0] += delta;}
    pub fn translate_y(&mut self, delta: f32){ self.position[1] += delta;}
    pub fn translate_z(&mut self, delta: f32){ self.position[2] += delta;}
    pub fn translate_forward(&mut self, delta: f32)
    {
	self.position += delta*self.forward;
    }
    pub fn translate_up(&mut self, delta: f32)
    {
	self.position += delta*self.up;
    }
    pub fn translate_side(&mut self, delta: f32)
    {
	self.position += delta*self.side;
    }


    /** Move the Objects of the scene according to the rotation of the Camera, Return the matrix of the view */
    /**
    returns the view matrix of the camera
    It will be used by the shaders to displace the objects of the scene to put them at the right place, with the right forward and size.
     */
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4]
    {

	let center = self.forward + self.position;
	let look_at = look_at(&self.position, &center, &self.up);
	let view_matrix = look_at.as_ref();
	*view_matrix 

    }

    /// Return the matrix of the perspective
    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4]
    {
        //let (width, height) = target.get_dimensions();
	let perspective = Perspective3::new(
	    self.aspect_ratio,
	    self.fov,
	    self.projection.znear,
 	    self.projection.zfar
	);
	
	let perspective_matrix = perspective.as_matrix().as_ref() ;
	*perspective_matrix
    }

    pub fn update_aspect_ratio(&mut self, gr: &Graphical)
    {
        let (w, h) = gr.display.display.get_framebuffer_dimensions();
        self.set_aspect_ratio(w as f32, h as f32);
    }
}
