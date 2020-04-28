use graphics::Graphical;

use graphics::RessourcesHolder;
use graphics::Scene;
use events_handling::{DevicesState, Event};

use base::{EngineError, Base};

use graphics::glium;
use glium::glutin;
use glutin::event_loop::{EventLoop, ControlFlow, EventLoopProxy};

use super::{GameState, GameStateStack, RenderBehavior, LogicBehavior, GameEvent};

use std::cell::RefCell;

use movable::Movable;

use imgui_winit_support::{HiDpiMode, WinitPlatform};
use imgui_glium_renderer::Renderer;
use imgui::{Context, Ui};






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
    pub event_loop_proxy: EventLoopProxy<GameEvent>,
    pub states: RefCell<GameStateStack>,

    gui_context: Context,
    gui_renderer: Renderer,
    gui_platform: WinitPlatform,

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



        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);
        let display = &gr.display.display;
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
        }


        /*
        éventuel truc à faire avec les fonts
         */
        let renderer = Renderer::init(&mut imgui, display).expect("Failed to initialize renderer");


        
        let movable = Movable::new(event_loop);
        Self
        {
            ressources: holder,
            graphic_engine: gr,
            base: base,
            devices: RefCell::new(DevicesState::new()),
            states: RefCell::new(GameStateStack::new()),
            event_loop: movable,
            event_loop_proxy: proxy,
            

            gui_context: imgui,
            gui_renderer: renderer,
            gui_platform: platform,
        }

    }


    /// renders the stored scene
    fn render(&mut self)
    {
        let mut frame = self.graphic_engine.frame();
        
        frame.clear();
        self.states.borrow_mut()
            .render(&self.graphic_engine,
                    &mut self.gui_renderer,
                    &mut frame,
                    &mut self.gui_context);
        
        frame.swap();
        
    }

    /// useless for now
        /// useless for now
    pub fn push_state(&mut self,
                      name: &str
    ) -> Result<(), base::EngineError>
    {
        if let Some(state) = self.states.get_mut()
            .loaded.remove(&name.to_string())
        {
            self.states.get_mut()
                .push(state);

        }
        else
        {
            let proto = self.states.get_mut().get_proto(name.to_string())?;
            let state = GameState::from_proto(self, &proto)?;
            self.states.get_mut()
                .push(state);
        }
        Ok(())
    }

    pub fn register_state(
        &mut self,
        name: &str,
        scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
        run_logic: fn(&mut GameState, &DevicesState),
        run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
        render_behavior: RenderBehavior,
        logic_behavior: LogicBehavior,

    )
    {
        self.states.get_mut()
            .register(
                name,
                scene_builder,
                run_gui,
                run_logic,
                render_behavior,
                logic_behavior)
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
                    GameEvent::Push(state_name) =>
                    {
                        self.push_state(
                            &state_name
                        ).unwrap();
                    }
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
                     // gui events
                     {
                         let gl_window = self.graphic_engine.display.display.gl_window();
                         self.gui_platform.handle_event(
                             self.gui_context.io_mut(),
                             gl_window.window(),
                             &event);
                     }
                     
                     // inputs
                     if let Some(ev) = Event::parse_relevant(event)
                     {
                         *control_flow = self.handle_event(ev);
                     }
                     
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


                         self.render();
                         render_date = now + delay;
                     }

                 });
    }
    
}
