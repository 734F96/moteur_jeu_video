use graphics::Graphical;

use graphics::RessourcesHolder;
use graphics::{Scene} ;
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

use sounds::{OneSound,SoundRessource};

use specs::{Dispatcher, World};


use std::collections::HashMap;




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

    pub sounds_played :HashMap<String,OneSound>,
    vol: f32,

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
            sounds_played: HashMap::new(),
            vol:0.0,

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
		    &self.ressources,
                    &mut self.gui_renderer,
                    &mut frame,
                    &mut self.gui_context);
        
        frame.swap();

    }

    pub fn load_state(&mut self,
		      name: &str) -> Result<(), base::EngineError>
    {
        let proto = self.states.get_mut()
	    .get_proto(name.to_string())?;
        let state = GameState::from_proto(self, &proto)?;
        self.states.get_mut()
            .loaded.insert(name.to_string(), state);
	Ok(())
	
    }
    

    pub fn push_state(&mut self,
                      name: &str) -> Result<(), base::EngineError>
    {
        if let Some(state) = self.states.get_mut()
            .loaded.remove(&name.to_string())
        {
            self.states.get_mut()
                .push(state);

        }
        else
        {
	    self.load_state(name)?;
	    self.push_state(name)?;
        }
        Ok(())
    }

    pub fn register_state(
        &mut self,
        name: &str,
        scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
        run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
        render_behavior: RenderBehavior,
        logic_behavior: LogicBehavior,
	init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)

    )
    {
        self.states.get_mut()
            .register(
                name,
                scene_builder,
                run_gui,
                render_behavior,
                logic_behavior,
		init)
    }
    
    fn pop_state(&self, n_to_pop: usize)
    {
        if n_to_pop > 0
        {
            self.states.borrow_mut().pop();
            self.pop_state(n_to_pop-1);
        }
    }


    //play sound entirely
    fn play_sound(&mut self, name: String, position: Option<[f32; 3]>)
    {
       let music_data =self.ressources.sounds_datas.get(name.as_str());
       match music_data{
            None => {},
            Some(music_data) => {
                 let music = OneSound::new_from_data(SoundRessource::new_from_data(music_data));
                 match music {
                      Ok(mut music) => {
                            let mut v= self.vol;
                            while v>0.0
                            {
                                music.up_volume();
                               v=v-1.0;
                            }

			    while v<0.0
                            {
                                music.down_volume();
                                v=v+1.0;
                            }
                            match position{
	                           None => {},
	                           Some(position) => music.give_position(position)
                             }

		             music.play_all();
                             //add in the hashmap of played_sound
                             let mut name_music=name.clone();
                             while self.sounds_played.contains_key(&name_music)
                                 {name_music.push('1')}
                             self.sounds_played.insert(name_music,music);
                       },
                      Err(_e) => {}
                }
            }
       }      
      
    }    


    // play sound with time limit
    // duration == none -> play the sound to infinity -> set the end fielf of OneSound to -2
    // duration == some -> play the sound duration sec
    fn play_sound_time_limit(&mut self,name: String, duration: Option<f32>,position: Option<[f32; 3]>){

       let music_data =self.ressources.sounds_datas.get(name.as_str());
       match music_data{
            None => {},
            Some(music_data) => {
                 let music = OneSound::new_from_data(SoundRessource::new_from_data(music_data));
                 match music{
                       Ok(mut music) => {
                           let mut v= self.vol;
                           while v>0.0
                           {
                               music.up_volume();
                              v=v-1.0;
                           }

			   while v<0.0
                           {
                               music.down_volume();
                               v=v+1.0;
                           }
                             match position{
	                           None => {},
		                   Some(position) => music.give_position(position)
	                     } 
			     
                             match duration{
                                   Some(duration) => { 
                                            music.set_end(duration);
                                            music.play_all(); 
                                   },
                                   None => {
					    music.set_end(-2.);
                                            music.play_all();}
                             }
                            //add in the hashmap of played_sound
                            let mut name_music=name.clone();
                            while self.sounds_played.contains_key(&name_music)
                                {name_music.push('1')}
                            self.sounds_played.insert(name_music,music);
                     },
                     Err(_e)=> {}     
                }
            }
       } 





   // down the volume of all played sounds
   fn down_volume(&mut self)
   {
      self.vol=self.vol-1.0;
      for (_name,sound) in self.sounds_played.iter_mut()
      {
         sound.down_volume();
      }
   } 

   // up the volume of all played sound
   fn up_volume(&mut self)
   {
      self.vol=self.vol+1.0;
      for (_name,sound) in self.sounds_played.iter_mut()
      {
         sound.up_volume();
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
                    },

                    GameEvent::PlaySound(name,position) => self.play_sound(name,position),
    		    GameEvent::PlaySoundTimeLimit(name,duration,position) => self.play_sound_time_limit(name,duration,position),
                    GameEvent::DownVolume()=>self.down_volume(),
                    GameEvent::UpVolume()=>self.up_volume()

                }
            }
            _ => ()
        };
        ControlFlow::Poll

    }

    /// Initialize and runs the game
    pub fn run(mut self, fps: u64) -> Result<(), base::EngineError>
    {
	
        let mut now = std::time::Instant::now();
        let mut render_date = std::time::Instant::now();

	let delay = std::time::Duration::from_millis(1000/fps);

	
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
			 
			 // takes about 99% of the time
                         self.render();
			 
                         render_date = now + delay;
                     }
                 
                     let name_to_pop : Vec<_> = self.sounds_played.iter().filter(|(_name,sound)|
                     (sound.end == (-1. as f32) && !sound.is_playing()) || 
                     (sound.end == (-2. as f32) && !sound.is_playing()) ||
                     (sound.end >0.0 && sound.end == sound.start.elapsed().as_secs() as f32)
                     ).map(|(name,_sound)| name.clone()).collect();
                      
		    for name in name_to_pop {
                         let sound = self.sounds_played.get_mut(&name);
                         match sound{
                            None => {},
                            Some(sound) => {
                                 if sound.end ==(-2. as f32)
                                 {sound.play_all();}
                                 else
                                 {
                                    if sound.end >0.0  
                                    {sound.stop();}
                                    self.sounds_played.remove(&name);
                                 }
                            }
                         }
                    }
                


		     
                 });
    }
    
}
