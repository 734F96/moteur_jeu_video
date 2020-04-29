extern crate moteur_jeu_video;

use moteur_jeu_video::prelude::*;

fn make_main_scene(
    game: &mut Game
) -> Result<Scene, EngineError>
{
    let disp = &game.graphic_engine.display;
    let holder = &mut game.ressources;
    let base = &game.base;
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;

    let sphere = holder.get_object("transparent_sphere", "Sphere").unwrap();
    let map_position = vec![
        Similarity
        {
            world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
        }
    ];
    
    

 
    
    // le buffer d'instanciation pour les cubes
    let instances = (0..40).map(|i| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random(), rand::random()),
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>();

    
    let mut scene = Scene::new(&disp);

    scene.add(vec![sphere], instances);

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
    let holder = &mut game.ressources;
    let base = &game.base;
    
    holder.add_tile(&disp, &base, "edgytet.png")?;
    
    let tile = holder.get_tile("edgytet", &disp)?;

    let tile_position = vec![Similarity {
        world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
    }];
    
    let mut scene = Scene::new(&disp);

    scene.add(vec![tile], tile_position);

    Ok(scene)
}

fn new_transformation((tx, ty, tz): (f32, f32, f32),
                      (rx, ry, rz): (f32, f32, f32), scale: f32) -> [[f32; 4]; 4]
{
    let rot =
        rotation(rx, &vec3(1., 0., 0.)) *
        rotation(ry, &vec3(0., 1., 0.)) *
        rotation(rz, &vec3(0., 0., 1.));
    let trans = translation(&vec3(tx, ty, tz));
    let resize = TMat4::from_diagonal(&vec4(scale, scale, scale, 1.));
    *(trans*rot*resize).as_ref()
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
                        LogicBehavior::Superpose);
    game.register_state("menu state",
                        make_menu_scene,
                        false,
                        menu_logic,
                        Some(render_gui),
                        RenderBehavior::Superpose,
                        LogicBehavior::Blocking);
    game.push_state("main state")?;
    game.load_state("menu state")?;
    println!("{:?}", game.ressources);
    
    game.run()

}

