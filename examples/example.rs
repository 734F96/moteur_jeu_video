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
    Component,
    NullStorage
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

use nalgebra::normalize;


fn make_main_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    let holder = &mut game.ressources;
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;

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

    

    holder.add_parameters(Params::new().with_transparency(true), "Sphere");

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
        .size([600.0, 400.0], Condition::FirstUseEver)
        .movable(false)
        .no_decoration()
        .build(&ui, || {            
            ui.same_line(275.0);
            ui.text(im_str!("Game menu"));
            ui.dummy([0.0, 5.0]);
            ui.new_line();
            ui.same_line(125.0);
            if ui.button(im_str!("Back to the game"), [350.0, 36.0])
            {
                proxy.send_event(GameEvent::Pop(1));
            };
            ui.dummy([0.0, 5.0]);
            ui.new_line();
            ui.same_line(125.0);
            if ui.button(im_str!("Click 1"), [150.0, 36.0])
            {
                proxy.send_event(GameEvent::QuitRequested);
            };
            ui.same_line_with_spacing(275.0, 50.0);
            
            if ui.button(im_str!("Click 2"), [150.0, 36.0])
            {
                proxy.send_event(GameEvent::QuitRequested);
            };
            ui.dummy([0.0, 5.0]);
            ui.new_line();
            ui.same_line(125.0);
            if ui.button(im_str!("Click 3"), [150.0, 36.0])
            {
                proxy.send_event(GameEvent::QuitRequested);
            };
            ui.same_line_with_spacing(275.0, 50.0);
            
            if ui.button(im_str!("Click 4"), [150.0, 36.0])
            {
                proxy.send_event(GameEvent::QuitRequested);
            };
            ui.dummy([0.0, 5.0]);
            ui.new_line();
            ui.same_line(125.0);
            if ui.button(im_str!("Quit the game"), [350.0, 36.0])
            {
                proxy.send_event(GameEvent::QuitRequested);
            };
            
           

            
        });

}
fn init_game(mut world: World, ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
{
    world.register::<Spatial>();
    world.register::<Model>();
    world.register::<Lighting>();
    world.register::<PhysicComponent>();
    world.insert(DevicesState::default());
    world.register::<ControledComp>();
    world.insert(Camera::default());

    let mut physics = Physics::default();
    
    let sphere = Model(ressources.get_object("transparent_sphere", "Sphere").unwrap());

    for _ in 0..50
    {
        let spatial = Spatial
        {
            pos: vec3(rand::random(), rand::random(), rand::random()),
            rot: vec3(rand::random(), rand::random(), rand::random()),
            scale: 0.001
        };

        world.create_entity()
        .with(spatial)
        .with(sphere)
        .build();
    }

    let map_elements = Model(ressources.get_whole_content("saloon").unwrap());
    let zero = Spatial
    {
        pos: vec3(0., 0., 0.),
        rot: vec3(0., 0., 0.),
        scale: 1.
    };

    world.create_entity()
	.with(zero)
	.with(map_elements)
    .build();
    
    let porte_chambre = Model(ressources.get_whole_content("porte_chambre").unwrap());
    let portes_chambres_positions = vec! [
        Spatial { pos: vec3(-19.3022, 3.41965, -17.4815), rot: vec3(0., 0., 0.), scale: 1. },
        Spatial { pos: vec3(-15.5513, 3.41965, -17.4815), rot: vec3(0., 0., 0.), scale: 1. },
        Spatial { pos: vec3(-10.5668, 3.41965, -17.4815), rot: vec3(0., 0.6981, 0.), scale: 1. },
    ];

    for position in portes_chambres_positions.iter()
    {   
        world.create_entity()
        .with(*position)
        .with(porte_chambre)
        .build();
    }
    
    let porte_entree = Model(ressources.get_whole_content("porte_entree").unwrap());
    let portes_entree_positions = vec! [
        Spatial { pos: vec3(-9.64833, 1.46962, -8.76043), rot: vec3(0., 0.7853, 0.), scale: 1. },
        Spatial { pos: vec3(-8.71997, 1.46962, -9.68726), rot: vec3(0., -2.3561, 0.), scale: 1.  },
    ];

    for position in portes_entree_positions.iter()
    {   
        world.create_entity()
        .with(*position)
        .with(porte_entree)
        .build();
    }

    let table = Model(ressources.get_whole_content("table").unwrap());
    let obj_table = ressources.get_by_handle(table.0);
    let table_trimesh = make_trimesh(&obj_table);
    let tables_positions = vec! [
        Spatial { pos: vec3(-14.6168, 0.333457, -12.643), rot: vec3(0., -0.33592, 0.), scale: 1. },
        Spatial { pos: vec3(-10.5536, 0.360777, -12.879), rot: vec3(0., 0.94535 , 0.), scale:1.  },
        Spatial { pos: vec3(-12.5902, 0.360777, -10.1726), rot: vec3(0., 0.28788 , 0.), scale:1.  },
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
 
    let lit_double = Model(ressources.get_whole_content("lit_double").unwrap());
    let lits_doubles_positions = vec! [
        Spatial { pos: vec3(-13.8841, 3.27735, -19.7949), rot: vec3(0., -1.5707, 0.), scale:1. },
        Spatial { pos: vec3(-19.6265, 3.27735, -19.7949), rot: vec3(0., 1.5707 , 0.), scale:1.  },
        Spatial { pos: vec3(-11.0315, 3.27735, -19.7949), rot: vec3(0., 1.5707, 0.), scale:1. },
    ];

    for position in lits_doubles_positions.iter()
    {   
        world.create_entity()
        .with(*position)
        .with(lit_double)
        .build();
    }
  
    let chaise = Model(ressources.get_whole_content("chaise").unwrap());
    let chaises_positions = vec! [
        Spatial { pos: vec3(-14.714, 0.325766, -11.6007), rot: vec3(0., 3.1415, 0.), scale:1. },
        Spatial { pos: vec3(-13.6238, 0.325766, -13.0231), rot: vec3(0., -1.22495, 0.), scale:1. },
        Spatial { pos: vec3(-15.3367, 0.325766, -13.4179), rot: vec3(0., 0.72583, 0.), scale:1. },
    ];

    for position in chaises_positions.iter()
    {   
        world.create_entity()
        .with(*position)
        .with(chaise)
        .build();
    }

    let tabourets = Model(ressources.get_whole_content("tabourets").unwrap());
    let tabourets_positions = vec! [
        Spatial { pos: vec3(-9.5536, 0.360777, -12.879), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-11.5536, 0.360777, -12.879), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.5536, 0.360777, -11.879), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.5536, 0.360777, -13.879), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-11.5902, 0.360777, -10.1726), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-13.5902, 0.360777, -10.1726), rot: vec3(0., 0., 0.),scale: 1. },
        Spatial { pos: vec3(-12.5902, 0.360777, -9.1726), rot: vec3(0., 0., 0.),scale: 1. },
        Spatial { pos: vec3(-12.5902, 0.360777, -11.1726), rot: vec3(0., 0., 0.), scale:1. },
    ];

    for position in tabourets_positions.iter()
    {   
        world.create_entity()
        .with(*position)
        .with(tabourets)
        .build();
    }

    let verres = Model(ressources.get_whole_content("verres").unwrap());
    let verres_positions = vec! [
        Spatial { pos: vec3(-10.4869, 1.2616, -12.4206), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-11.0091, 1.2616, -13.0586), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-10.0782, 1.2616, -13.108), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.2123, 1.2616, -10.3266), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.4423, 1.2616, -9.69564), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.9243, 1.2616, -9.96789), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.7213, 1.2616, -10.7131), rot: vec3(0., 0., 0.), scale:1. },
    ];

    for position in verres_positions.iter()
    {   
        world.create_entity()
        .with(*position)
        .with(verres)
        .build();
    }

    let bouteille = Model(ressources.get_whole_content("bouteille").unwrap()); // Model
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
        Spatial { pos: vec3(-12.5093, 1.2616, -10.2678), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.7289, 1.2616, -10.2876), rot: vec3(0., 0., 0.), scale:1. },
        Spatial { pos: vec3(-12.613, 1.2616, -10.0908), rot: vec3(0., 0., 0.), scale:1. },
    ];

    let light = Light::NonDirectional
	(
	    0.4,
	    [1., 0.8, 0.2]
	);

    
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
	    .with(Lighting(light))
	    .with(phy)
        .build();
    }

    let teto = Model(ressources.get_object("teto", "Lat式改変テト_mesh_Lat式改変テト").unwrap());

    let light = Light::NonDirectional
	(
	    0.004,
	    [1., 0.8, 0.2]
	);
    world.create_entity()
	.with(Lighting(light))
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
	    let speed = 0.40; // parce que pourquoi pas.

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
	    if devices.key_pressed(Key::Escape) {
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
    type SystemData = (Write<'a, Physics>, WriteStorage<'a, Spatial>, ReadStorage<'a, PhysicComponent>);

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




fn init_menu(mut world: World, ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
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
Un exemple simple avec un état de jeu et un état pour le menu.
Le menu bloque le jeu quand il est en place, mais le jeu s'affiche toujours même
si le menu est par-dessus.
Le jeu n'as pas de GUI, le menu si.

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
    
    game.run(10) // fps

}

