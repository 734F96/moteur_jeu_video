use super::{GameEvent, Game};
use graphics::{Scene, Graphical, Frame};
use events_handling::DevicesState;
use graphics::glium::glutin::event_loop::EventLoopProxy;

use imgui::{Ui, Context};
use imgui_glium_renderer::Renderer;

use base::EngineError;

use std::collections::HashMap;

pub struct GameState
{
    pub name: String,
    pub scene: Scene,
    gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    pub logic: fn(&mut GameState, &DevicesState),
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,
    proxy: EventLoopProxy<GameEvent>
}



impl GameState
{
    pub fn new(name: String,
               scene: Scene,
               logic: fn(&mut GameState, &DevicesState),
               render_behavior: RenderBehavior,
               logic_behavior: LogicBehavior,
               gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
               proxy: EventLoopProxy<GameEvent>) -> Self
    {
        Self
        {
            name: name,
            scene: scene,
            logic: logic,
            render_behavior: render_behavior,
            gui: gui,
            logic_behavior: logic_behavior,
            proxy: proxy
        }
    }
    
    pub fn from_proto(
        game: &mut Game,
        proto: &ProtoState) -> Result<Self, EngineError>
    {
        Ok(Self
           {
               name: proto.name.clone(),
               scene: (proto.scene_builder)(game)?,
               logic: proto.run_logic,
               render_behavior: proto.render_behavior,
               gui: proto.run_gui,
               logic_behavior: proto.logic_behavior,
               proxy: game.event_loop_proxy.clone()
           })
    }

    pub fn send_event(&self, user_event: GameEvent)
    {
        match self.proxy.send_event(user_event)
        {
            Err(_) => panic!("Cannot send user event: Event Loop terminated"),
            _ => ()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderBehavior
{
    NoRender,
    Superpose,
    Blocking
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicBehavior
{
    Superpose,
    Blocking
}

pub struct GameStateStack
{
    stack: Vec<GameState>,
    register: HashMap<String, ProtoState>,
    pub loaded: HashMap<String, GameState>
}


impl GameStateStack
{
    pub fn new() -> Self
    {
        Self
        {
            stack: Vec::new(),
            register: HashMap::new(),
            loaded: HashMap::new()
        }
    }

    pub fn register_proto(
        &mut self,
        name: &str,
        proto: ProtoState      
    )
    {
        self.register.insert(name.to_string(), proto);
    }

    pub fn get_proto(&self, name: String) -> Result<ProtoState, EngineError>
    {
        match self.register.get(&name)
        {
            Some(proto) => Ok(proto.clone()),
            None => EngineError::new(&format!("Game State {} not registered", name))
        }
    }
    
    pub fn register(
        &mut self,
        name: &str,
        scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
        run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
        run_logic: fn(&mut GameState, &DevicesState),
        render_behavior: RenderBehavior,
        logic_behavior: LogicBehavior,
    
    )
    {
        let name = name.to_string();
        self.register.insert(
            name.clone(),
            ProtoState
            {
                name: name,
                scene_builder: scene_builder,
                run_gui: run_gui,
                run_logic: run_logic,
                render_behavior: render_behavior,
                logic_behavior: logic_behavior
            });
    }
/*
    pub fn push_registered(&mut self,
                           name: String,
                           game: &mut Game) -> Result<(), EngineError>
    {
        if let Some(state) = self.loaded.remove(&name)
        {
            self.stack.push(state);
            Ok(())
        }
        else if let Some(proto) = self.register.get(&name)
        {
            self.stack.push(
                GameState::from_proto(
                    game,
                    proto
                )?
            );
            Ok(())
        }
        else
        {
            EngineError::new("Could not push state into stack")
        }
    }
  */  
    pub fn push(&mut self, state: GameState)
    {
        self.stack.push(state);
    }

    pub fn pop(&mut self)
    {
        if let Some(state) = self.stack.pop()
        {
            let name = state.name.clone();
            println!("Storing state '{}'", name);
            self.loaded.insert(name, state);
        };
    }

    pub fn iter(&self) -> std::slice::Iter<GameState>
    {
        self.stack.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<GameState>
    {
        self.stack.iter_mut()
    }
    
    pub fn render(&mut self,
                  gr: &Graphical,
                  gui_renderer: &mut Renderer,
                  frame: &mut Frame,
                  gui_context: &mut Context)
    {
        let first_block = self.iter()
            .rposition(|state| state.render_behavior == RenderBehavior::Blocking);
        let to_skip = match first_block
        {
            None => 0,
            Some(pos) => pos
        };
        for state in self.iter_mut().skip(to_skip)
            .filter(|state| state.render_behavior != RenderBehavior::NoRender)
        {
            state.scene.render(gr, frame);

        // gui
            if let Some(gui) = state.gui
            {
                let mut ui = gui_context.frame();
                
                (gui)(&mut ui, &state.proxy);
                
                let draw_data = ui.render();
                gui_renderer
                    .render(&mut frame.frame, draw_data)
                    .expect("Rendering failed GUI on frame");
            }
        }
    }

    
    pub fn logic(&mut self, devices: &DevicesState)
    {
        let first_block = self.iter()
            .rposition(|state| state.logic_behavior == LogicBehavior::Blocking);
        let to_skip = match first_block
        {
            None => 0,
            Some(pos) => pos
        };
        for state in self.iter_mut().skip(to_skip)
        {
            (state.logic)(state, devices);
        }
    }
}

#[derive(Clone)]
pub struct ProtoState
{
    name: String,
    scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
    run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    run_logic: fn(&mut GameState, &DevicesState),
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,
   
}
