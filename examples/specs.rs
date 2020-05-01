extern crate moteur_jeu_video;

use moteur_jeu_video::prelude::*;
use specs::
{
    World,
    WorldExt,
    DispatcherBuilder,
    Dispatcher,
    Builder,
};

use moteur_jeu_video::
{
    Spatial,
    Model,
};

use graphics::RessourcesHolder;






fn make_main_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    let holder = &mut game.ressources;
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(disp, "maison.obj", &ressources_path)?;
    
    let mut scene = Scene::new(&disp);


    for _ in 0..10
    {
	scene.add_light(
	    Light::Point(
	    
		1.,
		[rand::random::<f32>(); 3],
		[rand::random::<f32>(); 3]
	    )
	);
    }    

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

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);
    let sensibility = 0.005;
    let speed = 0.8; // parce que pourquoi pas.

    let (mouse_x, mouse_y) = devices.mouse_motion();
    camera_rot[1] -= (mouse_x as f32) * sensibility;
    camera_rot[0] -= (mouse_y as f32) * sensibility;

    if devices.key_continuous(Key::Q) {
        camera_pos[2] = camera_pos[2] - speed;
    }
    if devices.key_continuous(Key::D) {
        camera_pos[2] = camera_pos[2] + speed;
    }
    if devices.key_continuous(Key::Z) {
        camera_pos[0] = camera_pos[0] + speed;
    }
    if devices.key_continuous(Key::S) {
        camera_pos[0] = camera_pos[0] - speed;
    }
    if devices.key_pressed(Key::Escape) {
        game_state.send_event(GameEvent::Push("menu state".to_string()));
    }
    game_state.scene.camera.relative_move(camera_pos);
    game_state.scene.camera.rotation(camera_rot.clone());
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

    let sphere = Model(ressources.get_object("transparent_sphere", "Sphere").unwrap());
    for _ in 0..400
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

    
    let dispatcher = DispatcherBuilder::new()
	.build();
    
    (world, dispatcher)
}

fn init_menu(ressources: &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
{
    let mut world = World::new();
    world.register::<Spatial>();
    world.register::<Model>();

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
    
    game.run()

}

