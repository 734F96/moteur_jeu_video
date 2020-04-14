#[macro_use]
extern crate gui;
extern crate graphics;
extern crate events_handling;
extern crate glium;
extern crate glutin;
extern crate image;
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_sys;

//use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase};
use imgui::*;
use imgui_glium_renderer::Renderer;
use graphics::engine::*;
use base::Base;
use base::EngineError;

use events_handling::{EventsHandler, Key};

use std::{thread, time};

fn main() -> Result<(), EngineError>
{   
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
        let mut frame = graphics.frame();
        let mut ui = imgui.frame();
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        frame.draw_2D(&graphics, (0., 0., 0.5, 0.5), 0.);
        frame.show();
    }
    /*

    let system = support::init(file!());
    system.main_loop(move |_, ui| { // ui is imgui.frame();
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
    */
    Ok(())
}