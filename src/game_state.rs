use super::{GameEvent, Game, Model, Spatial};
use graphics::{Scene, Graphical, Frame, RessourcesHolder};
use events_handling::DevicesState;
use graphics::
{
    glium::glutin::event_loop::EventLoopProxy,
    Similarity
};

use imgui::{Ui, Context};
use imgui_glium_renderer::Renderer;

use base::EngineError;

use physics::Physics;

use std::collections::HashMap;

use physics::nphysics3d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    world::{DefaultMechanicalWorld, DefaultGeometricalWorld}

};
use physics::make_objects;
use graphics::nalgebra::Vector3;

use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;


use specs::
{World,
 Dispatcher,
 WorldExt,
 join::Join,
 join::ParJoin
};


pub struct GameState
{
    pub name: String,
    pub scene: Scene,
    pub physics: Option<Physics>,
    gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    logic: fn(&mut GameState, &DevicesState),
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,
    proxy: EventLoopProxy<GameEvent>,

    pub world: World,
    dispatcher: Dispatcher<'static, 'static>
}



impl GameState
{
    
    fn new(name: String,
               scene: Scene,
               with_physics: bool,
               logic: fn(&mut GameState, &DevicesState),
               render_behavior: RenderBehavior,
               logic_behavior: LogicBehavior,
               gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
               proxy: EventLoopProxy<GameEvent>,
	   init: fn(&mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>),
	   ressources: &mut RessourcesHolder
    ) -> Self
    {
        let physics = if with_physics
        {
            // MechanicalWorld with a gravity vector
            let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

            let geometrical_world = DefaultGeometricalWorld::<f32>::new();
            let joint_constraints = DefaultJointConstraintSet::<f32>::new();
            let force_generators = DefaultForceGeneratorSet::<f32>::new();

            let obj_set = make_objects(&scene);

            // (bodies, colliders, coll_tab)
            let three_uplet = physics::build_rb_col(obj_set);

            // Where we store all the RigidBody object
            let bodies = three_uplet.0;

            // Where we store all the Collider object
            let colliders = three_uplet.1;

            // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
            let col_tab = three_uplet.2;

            Some(Physics::new(mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators, col_tab))
        }
        else
        {
            None
        };
	
	let (world, dispatcher) = init(ressources);
        Self
        {
            name: name,
            scene: scene,
            logic: logic,
            render_behavior: render_behavior,
            gui: gui,
            logic_behavior: logic_behavior,
            proxy: proxy,
            physics: physics,
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
                     proto.with_physics,
                     proto.run_logic,
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

    /// probably temporary function (will be in use as long as a Scene is used for render)
    pub fn update_scene(&mut self)
    {
	// pas d'instantiation pour l'instant (soon)
	let models_storage = self.world.read_storage::<Model>();
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
	let data: Vec<_> = instances.into_par_iter()
	    .map(|(model, inst)| (vec![*model], inst))
	    .collect();
	self.scene.objects = data;
	
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
        with_physics: bool,
        run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
        run_logic: fn(&mut GameState, &DevicesState),
        render_behavior: RenderBehavior,
        logic_behavior: LogicBehavior,
	init: fn(&mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
    
    )
    {
        let name = name.to_string();
        self.register.insert(
            name.clone(),
            ProtoState
            {
                name: name,
                with_physics: with_physics,
                scene_builder: scene_builder,
                run_gui: run_gui,
                run_logic: run_logic,
                render_behavior: render_behavior,
                logic_behavior: logic_behavior,
		init: init
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
            (state.logic)(state, devices);
	    state.dispatcher.dispatch(&mut state.world);
        }
    }
}

#[derive(Clone)]
pub struct ProtoState
{
    name: String,
    with_physics: bool,
    scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
    run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    run_logic: fn(&mut GameState, &DevicesState),
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,

    init: fn(&mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
   
}
