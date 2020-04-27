use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::glutin::event_loop::EventLoop;


/**
Wrapper around the "Display" struct from glium
*/
pub struct Display
{    
    pub display: glium::Display,
}


impl Display
{
    /// returns a Display using the given EventLoop
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self
    {	
	let wb = WindowBuilder::new();  
	let cb = ContextBuilder::new().with_depth_buffer(24); 
	Self
	{	  
	    display: glium::Display::new(wb, cb, event_loop).unwrap()   
	}
    }
}
