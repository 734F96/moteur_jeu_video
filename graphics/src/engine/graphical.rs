use super::{Frame, Display, ProgramsHolder};
use crate::ressource_handling::RessourcesHolder;
use base::Base;
use glium::glutin::event_loop::EventLoop;

/**
Owns the various components needed to display things on the screen.
*/
pub struct Graphical
{
    pub display: Display,
    pub program: ProgramsHolder,
}


impl Graphical
{
    /// Constructor of Graphical
    pub fn new<T>(event_loop: &EventLoop<T>, base: &Base, holder: &mut RessourcesHolder) -> Self
    {
	let display = Display::new(event_loop) ;
        let mut program = ProgramsHolder::new();
        program.update(&display, base, holder);
        Self
        {
            display: display,
	    program: program,
        }
    }

    /// Generates a Frame to be drawn onto
    pub fn frame(&self) -> Frame
    {
        Frame::new(self)
    }

}
