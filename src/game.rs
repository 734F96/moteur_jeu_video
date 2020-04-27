use graphics::{Graphical, Display};

use graphics::RessourcesHolder;
use graphics::Scene;
use events_handling::{DevicesState, Event};

use base::{EngineError, Base};

use graphics::glium;
use glium::glutin;
use glutin::event_loop::{EventLoop, ControlFlow, EventLoopProxy};

use super::{GameState, GameStateStack, RenderBehavior, LogicBehavior};

use std::cell::RefCell;
use std::sync::Arc;

use movable::Movable;

pub enum GameEvent
{
    QuitRequested,
    Pop(usize),
    Push(
        fn(&mut Game) -> Result<Scene, EngineError>,
        fn(&mut GameState, &DevicesState),
        RenderBehavior,
        LogicBehavior
    )
}




/**
The Game structure
It owns everything
*/
pub struct Game
{
    pub graphic_engine: Graphical,
    pub ressources: RessourcesHolder,
    pub base: Base,
    pub devices: RefCell<DevicesState>,
    event_loop: Movable<EventLoop<GameEvent>>,
    event_loop_proxy: EventLoopProxy<GameEvent>,
    pub states: RefCell<GameStateStack>,
}

impl Game
{
    pub fn new() -> Self
    {
        let event_loop = EventLoop::<GameEvent>::with_user_event();
        let base = Base::new();
        let mut holder = RessourcesHolder::new();
        let gr = Graphical::new(&event_loop, &base, &mut holder);
        let proxy = event_loop.create_proxy();

        let movable = Movable::new(event_loop);
        Self
        {
            ressources: holder,
            graphic_engine: gr,
            base: base,
            devices: RefCell::new(DevicesState::new()),
            states: RefCell::new(GameStateStack::new()),
            event_loop: movable,
            event_loop_proxy: proxy
        }

    }


    /// renders the stored scene
    fn render(&self)
    {
        let mut frame = self.graphic_engine.frame();

        frame.clear();
        self.states.borrow_mut().render(&self.graphic_engine, &mut frame);
        
        frame.swap();
        
    }

    /// useless for now
    fn init(&mut self) -> Result<(), base::EngineError>
    {
        /*
        let scene = make_scene(
            &self.graphic_engine.display,
            &mut self.ressources,
            &self.base
        )?;

        self.set_scene(scene);
*/
        Ok(())
    }

        /// useless for now
    pub fn push_state(&mut self,
                         scene_maker: fn(&mut Game) -> Result<Scene, EngineError>,
                         logic: fn(&mut GameState, &DevicesState),
                         render_behavior: RenderBehavior,
                         logic_behavior: LogicBehavior
    ) -> Result<(), base::EngineError>
    {
        let scene = scene_maker(self)?;
        let state = GameState::new(scene, logic,
                                   render_behavior,
                                   logic_behavior,
                                   self.event_loop_proxy.clone());
        self.states.borrow_mut().push(state);
        Ok(())
    }

    fn pop_state(&self, n_to_pop: usize)
    {
        if n_to_pop > 0
        {
            self.states.borrow_mut().pop();
            self.pop_state(n_to_pop-1);
        }
    }


    

    // maybe user defined
    fn handle_event(&mut self, event: Event<GameEvent>) -> ControlFlow
    {
//        let mut devices = self.devices.borrow_mut();
        match event {
            Event::KeyPressed(key) => {self.devices.get_mut().keyboard_pressed.insert(key);},
            Event::KeyReleased(key) => {self.devices.get_mut().keyboard_continuous.remove(&key);},
            Event::ButtonPressed(button) => {self.devices.get_mut().mouse_state.insert(button);},
            Event::ButtonReleased(button) => {self.devices.get_mut().mouse_state.remove(&button);},
            Event::MouseMove(x, y) => {
                let mut devices = self.devices.get_mut();
                devices.mouse_move = (devices.mouse_move.0+x, devices.mouse_move.1+y);
            },
            Event::ScrollMouse(x, y) => {
                let mut devices = self.devices.get_mut();
                    devices.mouse_scroll = (devices.mouse_scroll.0+x, devices.mouse_scroll.1+y);
            },
            Event::GameEvent(game_event) =>
            {
                match game_event
                {
                    GameEvent::QuitRequested => return ControlFlow::Exit,
                    GameEvent::Pop(n) => self.pop_state(n),
                    GameEvent::Push(scene_maker, logic, render_bhv, logic_bhv) =>
                        {self.push_state(scene_maker, logic, render_bhv, logic_bhv);}
                }
            }
            _ => ()
        };
        ControlFlow::Poll

    }

    /// Initialize and runs the game
    pub fn run(mut self) -> Result<(), base::EngineError>
    {

        let mut now = std::time::Instant::now();
        let mut render_date = std::time::Instant::now();
        // 30 fps
        let delay = std::time::Duration::from_millis(1000/30);
        
        self.event_loop.consume()
            .run(move |event, _, control_flow|
                 {
                     // inputs
                     if let Some(ev) = Event::parse_relevant(event)
                     {
                         *control_flow = self.handle_event(ev);
                     } 

                     // game logic
                                          

                     
                     // render
                     now = std::time::Instant::now();
                     if render_date < now
                     {
 
                     self.states.borrow_mut()
                         .logic(&self.devices.borrow());
                        {
                             let mut devices = self.devices.borrow_mut();
                             devices.clear();
                         }

                         
                         let delta = (now-render_date+delay).as_nanos();
                         //println!("{} fps ({} ns)", 1_000_000_000/(delta+1), delta);
                         self.render();
                         render_date = now + delay;
                     }

                 });
    }
    
}
