use super::camera::*;
use super::frame::*;
use super::programs::*;




/**
Wrapper around the "Display" struct from glium
*/
pub struct Display
{    
    pub display: glium::Display,
}


impl Display
{
   pub fn new(event_loop: &glutin::EventsLoop) -> Self
   {	
	let wb = glutin::WindowBuilder::new();  
	let cb = glutin::ContextBuilder::new().with_depth_buffer(24); 
	Self
	{	  
		display: glium::Display::new(wb, cb, event_loop).unwrap()   
	}
   }
}



/**
Owns the rendering parameters.
 */
pub struct Params<'a>
{
    pub parameters: glium::draw_parameters::DrawParameters<'a>,
}


impl<'a> Params<'a>
{
    pub fn new() -> Self
    {
	Self
        {  parameters : glium::DrawParameters                               // A BOUGER --> structure params
           	{
            		depth: glium::Depth {
                		test: glium::DepthTest::IfLess, // if the object is 
                		write: true, // alors on dessine
                		.. Default::default() // Others parameters initialised by default
            			},
            		.. Default::default()
           	}
	}

    }

    /**draw only the lines of the edges of the traingles which composed ours polygons.**/
    pub fn polygon_line (mut self) -> Self
    {
		self.parameters.polygon_mode= glium::draw_parameters::PolygonMode::Line;
		return self

    }

    
    /**draw only the traingles'points which composed ours polygons.**/
    pub fn polygon_point (mut self) -> Self
    {
		self.parameters.polygon_mode= glium::draw_parameters::PolygonMode::Point;
		return self

    }

    /**draw all the content of ours polygons.**/
    pub fn polygon_fill (mut self) -> Self
    {
		self.parameters.polygon_mode= glium::draw_parameters::PolygonMode::Fill;
		return self

    }

     /**color all the polygons with the color passed in arguments*/
    pub fn color_all (mut self,r: bool, g:bool ,b:bool ,a:bool)-> Self
    {
		self.parameters.color_mask= (r,g,b,a);
		return self
    }

 
    /**enable or disable the transparency**/
    pub fn with_transparency(mut self, activated:bool) -> Self
    {
	if activated
	{
		self.parameters.blend=glium::Blend::alpha_blending();
	}
	else {
            self.parameters.blend=glium::Blend::default();
        }
	return self
    }

    pub fn always_top(mut self) -> Self
    {
	self.parameters.depth.test = glium::DepthTest::Overwrite;
	self
    }
}



/**
Owns the various components needed to display things on the screen.
*/
pub struct Graphical<'a>
{
    pub parameters: Params<'a>,
    pub display: Display,
    pub program: Programs,
    pub camera: Camera,
}


impl<'a> Graphical<'a>
{
    /** Constructor of Graphical */
    pub fn new(event_loop: &glutin::EventsLoop) -> Self
    {
        //let event_loop = glutin::EventsLoop::new();                    

	let display = Display::new(event_loop) ;
	let params = Params::new();
        let program = Programs::new(&display) ;
        
        Self
        {
            parameters: params,
            display: display,
	    program: program,
            camera: Camera::new(2.0),
            //event_loop: event_loop
        }
    }

    pub fn frame(&mut self) -> Frame
    {
        Frame::new(self)
    }

    pub fn update_dimensions(&mut self)
    {
        let (w, h) = self.display.display.get_framebuffer_dimensions();
        self.camera.set_aspect_ratio(w as f32, h as f32);
    }

}
