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

use physics::{Physics, make_trimesh, ShapeType, RbData, ColData};

use nalgebra::{Translation, Rotation, normalize};


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

    let mut scene = Scene::new(&disp);


    Ok(scene)
}


fn make_menu_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    
    let mut scene = Scene::new(&disp);

    Ok(scene)
}



fn game_logic(game_state: &mut GameState,
              devices: &DevicesState)
{
/*
    ///////////////////
    // #################################################################################
    let mut physics = game_state.physics.as_mut().unwrap();
    let mut i = 0;
    physics.run();
    for object in game_state.scene.objects.iter_mut() {
        for similarity in object.1.iter_mut() {
            let homogenous = physics
                .colliders
                .get(physics.col_tab[i])
                .unwrap()
                .position()
                .to_homogeneous();
            let (_, _, scale) = similarity.deconstruct();
            similarity.world_transformation = *homogenous.as_ref();
            let (tra, rot, _) = similarity.deconstruct();
            *similarity = Similarity::new(tra, rot, scale);
            i += 1;
        }
    }
    // #################################################################################
*/


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
                proxy.send_event(GameEvent::QuitRequested);
            };

            ui.text(im_str!("Useless text"));
        });

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
	    .make_static(pos, rot, scale, true);
	
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


    for _ in 0..4
    {
	let radius = 30.;
	let pos = [(rand::random::<f32>()-0.5)*radius,
		   (rand::random::<f32>()-0.5)*radius,
		   (rand::random::<f32>()-0.5)*radius];
	let rot = [rand::random::<f32>(); 3];
	let light = Light::Point
	    (
		1000.,
		pos,
		rot
	    );
	world.create_entity()
	    .with(Lighting(light))
	    .with(Spatial
		  {
		      pos: vec3(pos[0], pos[1], pos[2]),
		      rot: vec3(rot[0], rot[1], rot[2]),
		      scale: 0.001
		  })
	    .with(teto)
	    .build();
    }    



    let light = Light::Point
	(
	    1000.,
	    [0.; 3],
	    [0.; 3],
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
		       WriteStorage<'a, Spatial>);
    fn run(&mut self, (mut camera, devices, controleds, mut spatials): Self::SystemData)
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

	for (spatial, _) in (&mut spatials, &controleds).join()
	{
	    spatial.pos = camera.position;
	}
	
    }
}


struct EventSendingSystem;

impl<'a> System<'a> for EventSendingSystem
{

    type SystemData = (Write<'a, EventSender>,
		       Read<'a, DevicesState>);
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

    type SystemData = (Write<'a, EventSender>,
		       Read<'a, DevicesState>);
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

	/*
	for (spatial, physic_comp) in (&spatial_st, &physical_st).join()
	{
	    
	    let physic_id = physic_comp.collider_id;

	    let mut pos = spatial.pos;
	    let mut rot = spatial.rot;

	    pos[2] += 0.1;
	    
	    let isometry =nalgebra::geometry::Isometry::<_, nalgebra::base::dimension::U3, nalgebra::geometry::UnitQuaternion<_>>::new(pos, rot);

	    physics
		.colliders
		.get_mut(physic_id)
		.unwrap()
		.set_position(isometry);

	}
*/
	physics.run();

	for (spatial, physic_comp) in (&mut spatial_st, &physical_st).join()
	{
	    
	    let physic_id = physic_comp.collider_id;

	    let isometry = physics
		.colliders
		.get(physic_id)
		.unwrap()
		.position().to_homogeneous();
	    let rotation = normalize(&(isometry * vec4(1., 1., 1., 0.)).xyz());
	    let translation = (isometry * vec4(0., 0., 0., 1.));
	    spatial.rot = rotation.xyz();
	    spatial.pos = translation.xyz()/translation[3];

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
    //    println!("{:?}", game.ressources);
    
    game.run(15)

}

