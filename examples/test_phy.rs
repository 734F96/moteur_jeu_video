extern crate moteur_jeu_video;

use moteur_jeu_video::prelude::*;
use specs::
{
    World,
    WorldExt,
    DispatcherBuilder,
    Dispatcher,
    Builder,
    Read,
    Write,
    System,
    ReadStorage,
    WriteStorage,
    Join,
    NullStorage,
    Component
};

use moteur_jeu_video::
{
    Spatial,
    Model,
    Lighting,
    PhysicComponent,
    EventSender
};

use graphics::
{
    Camera,
    RessourcesHolder
};

use physics::{Physics, make_trimesh};




fn make_main_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    let holder = &mut game.ressources;
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(disp, "sphere_smooth.obj", &ressources_path)?;
    holder.load_wavefront(disp, "all_objects_saloon.obj", &ressources_path)?;
    holder.load_wavefront(disp, "teto.obj", &ressources_path)?;
/*
    holder.load_wavefront(disp, "saloon.obj", &ressources_path)?;
    holder.load_wavefront(disp, "porte_chambre.obj", &ressources_path)?;
    holder.load_wavefront(disp, "porte_entree.obj", &ressources_path)?;
    holder.load_wavefront(disp, "table.obj", &ressources_path)?;
    holder.load_wavefront(disp, "lit_double.obj", &ressources_path)?;
    holder.load_wavefront(disp, "chaise.obj", &ressources_path)?;
    holder.load_wavefront(disp, "tabourets.obj", &ressources_path)?;
    holder.load_wavefront(disp, "verres.obj", &ressources_path)?;
    holder.load_wavefront(disp, "bouteille.obj", &ressources_path)?;
    holder.load_wavefront(disp, "teto.obj", &ressources_path)?;
*/
    

//    holder.add_parameters(Params::new().with_transparency(true), "Sphere");

    let scene = Scene::new(&disp);

    Ok(scene)
}



fn make_menu_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    
    let scene = Scene::new(&disp);

    Ok(scene)
}



fn render_gui(ui: &mut Ui, proxy: &EventLoopProxy<GameEvent>)
{
    Window::new(im_str!("Pause Menu"))
    .size([300.0, 110.0], Condition::FirstUseEver)
    .movable(false)
    .no_decoration()
    .build(&ui, || {
        if ui.button(im_str!("QUIT"), [60.0, 36.0])
        {
            proxy.send_event(GameEvent::QuitRequested).unwrap();
        };

        ui.text(im_str!("Useless text"));
    });

    use imgui::Slider;

    let mut val = 0;

    Slider::new(im_str!("Slider"), 0..=4)
	.build(&ui, &mut val);


}
    
fn init_game(mut world: World, ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
{
    world.register::<Spatial>();
    world.register::<Model>();
    world.register::<Lighting>();
    world.register::<PhysicComponent>();
    world.register::<ControledComp>();
    world.insert(DevicesState::default());

    let mut camera = Camera::default();

    
    camera.set_position(vec3(-18., 1., -10.));
    world.insert(camera);

    let mut physics = Physics::default();
    
    let sphere = Model(ressources.get_object("sphere_smooth", "Sphere").unwrap());
    for _ in 0..50
    {
        let spatial = Spatial
        {
                pos: vec3(rand::random(), rand::random(), rand::random()),
                rot: vec3(rand::random(), rand::random(), rand::random()),
                scale: 0.1
        };

        world.create_entity()
        .with(spatial)
        .with(sphere)
        .build();
    }
println!("MARCO");

    let bouteille = Model(ressources.get_object("all_objects_saloon", "bouteille_SM_Prop_Bottle_363_SM_Prop_Bottle_01").unwrap()); // Model

    println!("POLO");

    let obj_bouteille = ressources.get_by_handle(bouteille.0) ; // &Object
    let bouteille_trimesh = make_trimesh(&obj_bouteille) ;

    let bouteilles_positions = vec! [
        Spatial { pos: vec3(-14.1798, 1.47845, -15.2044), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-14.2691, 1.47845, -15.0703), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-13.1945, 1.48155, -15.2379), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-13.0485, 1.48304, -15.1097), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.075, 1.48645, -15.2669), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-9.7778, 1.48645, -15.1302), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.7084, 1.2616, -13.1072), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.675, 1.2616, -12.679), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.471, 1.2616, -12.9902), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.5093, 1.2616, -10.2678), rot: vec3(-0.7, 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.7289, 1.2616, -10.2876), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.613, 1.2616, -10.0908), rot: vec3(0., 0., 0.), scale:1. },
    ];


    for position in bouteilles_positions.iter()
    {
	let Spatial{pos, rot, scale} = position.clone();
        let physic_obj_bouteille = bouteille_trimesh
	    .make_dynamic(pos, rot, scale, true);
	
        let gen_index = physics.build_rigbd_col(&physic_obj_bouteille);

	    let phy = PhysicComponent
	    {
	        collider_id: gen_index,
	        shape: bouteille_trimesh.clone()
	    };
	
        world.create_entity()
        .with(*position)
        .with(bouteille)
	    .with(phy)
        .build();
    }


    let table = Model(ressources.get_object("all_objects_saloon", "table_SM_Prop_Table_3_SM_Prop_Table_01").unwrap());

    let obj_table = ressources.get_by_handle(table.0);
    let table_trimesh = make_trimesh(&obj_table);
    
    let tables_positions = vec! [
        Spatial { pos: vec3(-14.6168, 0.333457, -12.643), rot: vec3(0., 0., 0.), scale: 1. },
        Spatial { pos: vec3(-10.5536, 0.360777, -12.879), rot: vec3(0., 0. , 0.), scale:1.  },
        Spatial { pos: vec3(-12.5902, 0.360777, -10.1726), rot: vec3(0., 0. , 0.), scale:1.  },
    ];

    for position in tables_positions.iter()
    {
	let Spatial{pos, rot, scale} = position.clone();
        let physic_obj_table = table_trimesh
	    .make_static(pos, rot*std::f32::consts::PI, scale, true);
	
        let gen_index = physics.build_rigbd_col(&physic_obj_table);

	let phy = PhysicComponent
	{
	    collider_id: gen_index,
	    shape: table_trimesh.clone()
	};
	
	world.create_entity()
            .with(*position)
            .with(table)
	    .with(phy)
            .build();
    }



    
    let teto = Model(ressources.get_object("teto", "Lat式改変テト_mesh_Lat式改変テト").unwrap());


    for _ in 0..6
    {
	let radius = 30.;
	let pos = [(rand::random::<f32>()-0.5)*radius,
		   (rand::random::<f32>()-0.5)*radius,
		   (rand::random::<f32>()-0.5)*radius];
	let light = Light::NonDirectional
	    (
		10.,
		[1., 1., 0.]
	    );
	world.create_entity()
	    .with(Lighting(light))
	    .with(Spatial
		  {
		      pos: vec3(pos[0], pos[1], pos[2]),
		      rot: vec3(0., 0., 0.),
		      scale: 0.001
		  })
	    .with(teto)
	    .build();
    }


    let light = Light::NonDirectional
	(
	    10.,
	    [0., 0., 1.]
	);
    world.create_entity()
	.with(Lighting(light))
	.with(Spatial
	      {
		  pos: vec3(0., 0., 0.),
		  rot: vec3(0., 0., 0.),
		  scale: 0.01
	      })
	.with(teto)
	.build();

    
    let light = Light::NonDirectional
	(
	    10.,
	    [1., 1., 1.],
	);
    world.create_entity()
	.with(Lighting(light))
	.with(Spatial
	{
		pos: vec3(0., 0.,0.,),
		rot: vec3(0., 0.,0.,),
		scale: 0.001
	})
	.with(ControledComp)
	.with(teto)
	.build();

    let light = Light::NonDirectional
	(
	    0.4,
	    [0.01, 0., 0.],
	);
    
    world.create_entity()
	.with(Lighting(light))
	.build();
    


    let obj_sphere = ressources.get_by_handle(sphere.0);
    let trim = make_trimesh(&obj_sphere);
    let physic_obj = trim
	.make_kinematic(vec3(0., 0., 0.), vec3(0., 0., 0.), 0.1, false);	
    let gen_index = physics.build_rigbd_col(&physic_obj);

    let phy = PhysicComponent
    {
	collider_id: gen_index,
	shape: trim.clone()
    };
    
    world.create_entity()
        .with(Spatial{pos: vec3(0., 0., 0.), rot: vec3(0., 0., 0.), scale: 0.3})
        .with(sphere)
	.with(phy)
//	.with(ControledComp)
        .build();



    world.insert(physics);

    let dispatcher = DispatcherBuilder::new()
	.with(CameraSystem, "camera motion", &[])
	.with(EventSendingSystem, "event sending", &[])
	.with(PhysicSystem, "physics", &[])
	.build();
    
    (world, dispatcher)
}



#[derive(Default)]
struct ControledComp;
impl Component for ControledComp
{
    type Storage = NullStorage<Self>;
}



struct CameraSystem;

impl<'a> System<'a> for CameraSystem
{
    type SystemData = (Write<'a, Camera>,
		       Read<'a, DevicesState>,
		       ReadStorage<'a, ControledComp>,
		       WriteStorage<'a, Spatial>,
    		       ReadStorage<'a, PhysicComponent>,
		       Write<'a, Physics>);
    fn run(&mut self, (mut camera, devices, controleds, mut spatials, physical, mut physics): Self::SystemData)
    {
	    let sensibility = 0.003;
	    let speed = 0.40; // Used to adjust the camera speed

	    let (mouse_x, mouse_y) = devices.mouse_motion();

	    camera.rotate(
	        (mouse_x as f32) * sensibility,
	        (mouse_y as f32) * sensibility
	    );

	
	    if devices.key_continuous(Key::Q) {
            camera.translate_side(-speed);
	    }
	    if devices.key_continuous(Key::D) {
            camera.translate_side(speed);
	    }
	    if devices.key_continuous(Key::Z) {
            camera.translate_forward(speed);
	    }
	    if devices.key_continuous(Key::S) {
            camera.translate_forward(-speed);
	    }
	    if devices.key_continuous(Key::Space) {
	        camera.translate_y(speed);
	    }
	    if devices.key_continuous(Key::LShift) {
	        camera.translate_y(-speed);
	    }

	    for (spatial, _, mut maybe_phy) in (&mut spatials, &controleds, physical.maybe()).join()
	    {
	        spatial.pos = camera.position;
	        spatial.rot = camera.forward;
		maybe_phy.iter_mut().for_each(
		    |phy| {
			physics
			    .colliders
			    .get_mut(phy.collider_id)
			    .unwrap()
			    .set_position(nalgebra::geometry::Isometry::<_, nalgebra::base::dimension::U3, nalgebra::geometry::UnitQuaternion<_>>::translation(spatial.pos[0], spatial.pos[1], spatial.pos[2]));

		    })
	    }

    }
}



struct EventSendingSystem;

impl<'a> System<'a> for EventSendingSystem
{

    type SystemData = (Write<'a, EventSender>, Read<'a, DevicesState>);
    fn run(&mut self, (mut sender, devices): Self::SystemData)
    {
	if devices.key_pressed(Key::Escape) || devices.key_pressed(Key::N) {
            sender.push(GameEvent::Push("menu state".to_string()));
	    }
    }
}



struct MenuEventSystem;

impl<'a> System<'a> for MenuEventSystem
{

    type SystemData = (Write<'a, EventSender>, Read<'a, DevicesState>);
    fn run(&mut self, (mut sender, devices): Self::SystemData)
    {
	    if devices.key_pressed(Key::Escape) {
            sender.push(GameEvent::Pop(1));
	    }
    }
}



struct PhysicSystem;

impl<'a> System<'a> for PhysicSystem
{
    type SystemData = (Write<'a, Physics>,
		       WriteStorage<'a, Spatial>,
		       ReadStorage<'a, PhysicComponent>);

    fn run(&mut self, (mut physics, mut spatial_st, physical_st): Self::SystemData)
    {
	    physics.run();
	    for (spatial, physic_comp) in (&mut spatial_st, &physical_st).join()
	    {
	    
	        let physic_id = physic_comp.collider_id;

	        let isometry = physics
		    .colliders
		    .get(physic_id)
		    .unwrap()
		    .position();

	        spatial.rot = isometry.rotation.scaled_axis();
	        spatial.pos = isometry.translation.vector;
	    }
    }
}



fn init_menu(mut world: World, _ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
{
    world.register::<Spatial>();
    world.register::<Model>();
    world.register::<Lighting>();
    world.insert(DevicesState::default());
    world.insert(Camera::default());

    let dispatcher = DispatcherBuilder::new()
	.with(MenuEventSystem, "event sending", &[])
	.build();
   
    (world, dispatcher)
}



/*
A simple example with a game_state and a state for the menu.
The menu freezes the game when it's showed, but the game is still displayed even though the menu is over it.
The game doesn't have any GUI but the menu does.
*/
fn main() -> Result<(), EngineError>
{
    
    let mut game = Game::new();
    game.register_state("main state",
                        make_main_scene,
                        None,
                        RenderBehavior::Superpose,
                        LogicBehavior::Superpose,
			            init_game
    );

    game.register_state("menu state",
                        make_menu_scene,
                        Some(render_gui),
                        RenderBehavior::Superpose,
                        LogicBehavior::Blocking,
			            init_menu

    );

    game.push_state("main state")?;
    game.load_state("menu state")?;
    
    game.run(15) // Parameter is the number of update per second (physic and fps)
}
