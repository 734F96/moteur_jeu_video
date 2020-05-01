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
    System
};

use moteur_jeu_video::
{
    Spatial,
    Model,
    Lighting
};

use graphics::
{
    Camera,
    RessourcesHolder
};






fn make_main_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    let holder = &mut game.ressources;
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(disp, "maison.obj", &ressources_path)?;
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

    if devices.key_pressed(Key::Escape) {
        game_state.send_event(GameEvent::Push("menu state".to_string()));
    }
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

fn menu_logic(game_state: &mut GameState,
              devices: &DevicesState)
{

    if devices.key_pressed(Key::Escape) {
        game_state.send_event(GameEvent::Pop(1));
    }

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

fn init_game(ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
{
    let mut world = World::new();
    world.register::<Spatial>();
    world.register::<Model>();
    world.register::<Lighting>();
    world.insert(DevicesState::default());
    world.insert(Camera::default());

    
    let sphere = Model(ressources.get_object("transparent_sphere", "Sphere").unwrap());
    for _ in 0..50
    {
	let spatial =Spatial
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

    let zero = Spatial
    {
	pos: vec3(0., 0., 0.),
	rot: vec3(0., 0., 0.),
	scale: 1.
    };
    let maison = Model(ressources.get_object("maison", "SM_Bld_Saloon_01_27_SM_Bld_Saloon_01").unwrap());
    world.create_entity()
	.with(zero)
	.with(maison)
	.build();


    let teto = Model(ressources.get_object("teto", "Lat式改変テト_mesh_Lat式改変テト").unwrap());


    for _ in 0..5
    {
	let radius = 10.;
	let pos = [(rand::random::<f32>()-0.5)*radius,
		   (rand::random::<f32>()-0.5)*radius,
		   (rand::random::<f32>()-0.5)*radius];
	let rot = [rand::random::<f32>(); 3];
	let light = Light::Point
	    (
		1.,
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



    
    
    let dispatcher = DispatcherBuilder::new()
	.with(CameraSystem, "camera motion", &[])
	.build();
    
    (world, dispatcher)
}


struct CameraSystem;

impl<'a> System<'a> for CameraSystem
{
    type SystemData = (Write<'a, Camera>,
		       Read<'a, DevicesState>);
    fn run(&mut self, (mut camera, devices): Self::SystemData)
    {

	let sensibility = 0.003;
	let speed = 0.08; // parce que pourquoi pas.

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
    }
}



fn init_menu(ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
{
    let mut world = World::new();
    world.register::<Spatial>();
    world.register::<Model>();
    world.register::<Lighting>();
    world.insert(DevicesState::default());
    world.insert(Camera::default());

    let dispatcher = DispatcherBuilder::new()
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
                        false,
                        game_logic,
                        None,
                        RenderBehavior::Superpose,
                        LogicBehavior::Superpose,
			init_game
    );
    game.register_state("menu state",
                        make_menu_scene,
                        false,
                        menu_logic,
                        Some(render_gui),
                        RenderBehavior::Superpose,
                        LogicBehavior::Blocking,
			init_menu

    );
    game.push_state("main state")?;
    game.load_state("menu state")?;
//    println!("{:?}", game.ressources);
    
    game.run(30)

}

