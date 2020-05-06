use super::{GameEvent, Game, Model, Spatial, Lighting};
use graphics::{Scene, Graphical, Frame, RessourcesHolder};
use events_handling::DevicesState;
use graphics::
{
    glium::glutin::event_loop::EventLoopProxy,
    Similarity,
    Camera
};

use imgui::{Ui, Context};
use imgui_glium_renderer::Renderer;

use base::EngineError;



use std::collections::HashMap;

use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;


use specs::
{
    World,
    Dispatcher,
    WorldExt,
    join::Join,
    join::ParJoin
};


pub struct GameState
{
    pub name: String,
    pub scene: Scene, // to be removed
    gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,
    proxy: EventLoopProxy<GameEvent>, // to be removed (maybe)

    pub world: World,
    dispatcher: Dispatcher<'static, 'static> // game states never die
}



impl GameState
{
    
    fn new(name: String,
           scene: Scene,
           render_behavior: RenderBehavior,
           logic_behavior: LogicBehavior,
           gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
           proxy: EventLoopProxy<GameEvent>,
	   init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>),
	   ressources: &mut RessourcesHolder
    ) -> Self
    {
	let mut world = World::new();

	world.insert(EventSender::new());


	
	let (world, dispatcher) = init(world, ressources);
        Self
        {
            name: name,
            scene: scene,
            render_behavior: render_behavior,
            gui: gui,
            logic_behavior: logic_behavior,
            proxy: proxy,
	    world: world,
	    dispatcher: dispatcher
        }
    }
    
    pub fn from_proto(
        game: &mut Game,
        proto: &ProtoState) -> Result<Self, EngineError>
    {
        Ok(Self::new(proto.name.clone(),
                     (proto.scene_builder)(game)?,
                     proto.render_behavior,
                     proto.logic_behavior,
                     proto.run_gui,
                     game.event_loop_proxy.clone(),
		     proto.init,
		     &mut game.ressources))
    }

    pub fn send_event(&self, user_event: GameEvent)
    {
        match self.proxy.send_event(user_event)
        {
            Err(_) => panic!("Cannot send user event: Event Loop terminated"),
            _ => ()
        }
    }
    pub fn send_events(&self)
    {
	let mut sender = self.world.write_resource::<EventSender>();
	sender.send(&self.proxy);
    }

    /// probably temporary function (will be in use as long as a Scene is used for render)
    pub fn update_scene(&mut self)
    {
	// pas d'instantiation pour l'instant (soon)
	let models_storage = self.world.read_storage::<Model>();
	let light_storage = self.world.read_storage::<Lighting>();
	let spatial_storage = self.world.read_storage::<Spatial>();
	let instances = (&models_storage, &spatial_storage).par_join()
	    .fold(|| HashMap::new(), |mut instances, (Model(obj_handle), Spatial{pos, rot, scale})|
		  {
		      let similarity = Similarity::new(*pos, *rot, *scale);
		      match instances.get_mut(obj_handle)
		      {
			  None =>
			  {
			      instances.insert(obj_handle, vec![similarity]);
			  },
			  Some(v) =>
			  {
			      v.push(similarity);
			  }
		      };
		      instances
		  })
	    .reduce(
		|| HashMap::new(),
		|mut total, part|
		{
		    part.into_iter()
			.for_each(
			    |(obj_handle, mut vect)|
			    {
				match total.get_mut(obj_handle)
				{
				    None =>
				    {
					total.insert(obj_handle, vect);
				    },
				    Some(v) =>
				    {
					v.append(&mut vect);
				    }
				};
			    });
			total
		}
	    );
	let objects: Vec<_> = instances.into_par_iter()
	    .map(|(model, inst)| (vec![*model], inst))
	    .collect();
	self.scene.objects = objects;


	self.scene.lights.clear();

	for (Lighting(light), maybe_spatial) in (&light_storage, spatial_storage.maybe()).join()
	{
	    self.scene.lights.push(*light, maybe_spatial
				   .map(|spatial| ([spatial.pos[0],
						   spatial.pos[1],
						    spatial.pos[2], 0.],
						   [spatial.rot[0],
						    spatial.rot[1],
						    spatial.rot[2], 0.],)
				   ))
	}


	let camera = self.world.read_resource::<Camera>();
	self.scene.camera = *camera;
	
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
        render_behavior: RenderBehavior,
        logic_behavior: LogicBehavior,
	init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
    
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
                render_behavior: render_behavior,
                logic_behavior: logic_behavior,
		init: init
            });
    }

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
		  ressources: &RessourcesHolder,
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
	    state.update_scene();

	    // about 90% of the time spent
            state.scene.render(gr, ressources, frame);

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
	    state.world.insert((*devices).clone());
	    state.dispatcher.dispatch(&mut state.world);

	    state.send_events();
	    
        }
    }
}

#[derive(Clone)]
pub struct ProtoState
{
    name: String,
    
    scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
    run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>),

    render_behavior: RenderBehavior, // can make a trait instead
    logic_behavior: LogicBehavior, // can make a trait instead

   
}


/// a ressource to send events
#[derive(Debug, Default)]
pub struct EventSender(Vec<GameEvent>);

impl EventSender
{
    fn new() -> Self
    {
	Self(Vec::new())
    }
    pub fn push(&mut self, event: GameEvent)
    {
	self.0.push(event);
    }

    fn send(&mut self, proxy: &EventLoopProxy<GameEvent>)
    {
	self.0.drain(..).for_each(
	    |event|
	    match proxy.send_event(event)
	    {
		Err(_) => panic!("Cannot send user event: Event Loop terminated"),
		_ => ()		
	    }
	)
    }
    
    
}
