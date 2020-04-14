extern crate gui;
extern crate graphics;
extern crate events_handling;

use gui::Rectangle;
use graphics::engine::*;
use base::Base;
use base::EngineError;

use events_handling::{EventsHandler, Key, Button};

use std::{thread, time};



fn main() -> Result<(), EngineError>
{   
    /*
    let string: String = "Default".to_string();
    let _rect = Rectangle::new(1.0, 1.0, 2.0, 5.0, string);
    println!("{:#?}", _rect);
    */

    let mut event_loop = glutin::EventsLoop::new();
    let mut handler = EventsHandler::new(&mut event_loop);

    let step = time::Duration::from_millis(1000/60); // 60 fps

    let mut base = Base::new();
    let mut graphics = Graphical::new(&base.get_events_loop());
    loop 
    {
        thread::sleep(step);
        while !handler.update(){}
        let devices = handler.state();
        if devices.button_pressed(Button::RightClick)
        {
            println!("MOUSE BUTTONS: {:?}", devices.mouse_state);
        }
        let mut frame = graphics.frame();
        frame.draw_2D(&graphics, (-0.1, 0., 0.2, 0.1), 0.);
        frame.show();
    }
    
    Ok(())
}