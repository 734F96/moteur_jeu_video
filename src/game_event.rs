use crate::*;
use graphics::glium::glutin::event_loop::EventLoopProxy;
use base::EngineError;
use graphics::Scene;
use events_handling::DevicesState;
use imgui::Ui;


pub enum GameEvent
{
    QuitRequested,
    Pop(usize),
    Push(String)
}
