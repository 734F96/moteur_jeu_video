extern crate graphics;
extern crate nalgebra;
extern crate rand;
	

use base::{Base, EngineError};
use events_handling::{Key, Event, DevicesState};

use graphics::engine::*;
use graphics::misc::*;
use graphics::ressource_handling::*;

use nalgebra::base::*;
use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4};

use glium::glutin::event_loop::{EventLoop, ControlFlow};

use sounds::{OneSound};

use std::thread;


/**
The Game structure
It owns everything
*/
struct Game
{
    scene: Scene,
    graphic_engine: Graphical,
    ressources: RessourcesHolder,
    base: Base,
    devices: DevicesState,
    exit: bool,
    game_logic: fn(&mut Self)
}

impl Game
{
    fn new(logic: fn(&mut Self), event_loop: &EventLoop<()>) -> Self
    {
        let base = Base::new();
        let mut holder = RessourcesHolder::new();
        let gr = Graphical::new(event_loop, &base, &mut holder);

        Self
        {
            scene: Scene::new(),
            ressources: holder,
            graphic_engine: gr,
            base: base,
            devices: DevicesState::new(),
            exit: false,
            game_logic: logic
        }

    }

    fn set_scene(&mut self, scene: Scene)
    {
        self.scene = scene;
    }

    /// renders the stored scene
    fn render(&mut self)
    {
        self.graphic_engine.update_dimensions();
        let mut frame = self.graphic_engine.frame();

        frame.clear();
        self.scene.render(&self.graphic_engine, &mut frame);
        
        frame.swap();
        
    }

    /// useless for now
    fn init(&mut self) -> Result<(), base::EngineError>
    {
        let scene = make_scene(
            &self.graphic_engine.display,
            &mut self.ressources,
            &self.base
        )?;
        self.set_scene(scene);
        Ok(())
    }
    

    // maybe user defined
    fn handle_event(&mut self, event: Event)
    {
        match event {
            Event::KeyPressed(key) => {self.devices.keyboard_state.insert(key);},
            Event::KeyReleased(key) => {self.devices.keyboard_state.remove(&key);},
            Event::ButtonPressed(button) => {self.devices.mouse_state.insert(button);},
            Event::ButtonReleased(button) => {self.devices.mouse_state.remove(&button);},
            Event::MouseMove(x, y) => {self.devices.mouse_move = (self.devices.mouse_move.0+x, self.devices.mouse_move.1+y);}
            Event::ScrollMouse(x, y) => {self.devices.mouse_scroll = (self.devices.mouse_scroll.0+x, self.devices.mouse_scroll.1+y);},
            _ => ()
        }

    }

    /// Initialize and runs the game
    fn run(mut self, event_loop: EventLoop<()>) -> Result<(), base::EngineError>
    {
        self.init()?;

	let thread_music= thread::spawn(move || {
		let mut global_music = OneSound:: new("../sounds/examples/animal-crossing.wav");
    		global_music.play_all();       
        });
	
        let mut now = std::time::Instant::now();
        let mut render_date = std::time::Instant::now();
        // 30 fps
        let delay = std::time::Duration::from_millis(1000/30);
        
        event_loop
            .run(move |event, _, control_flow|
                 {

                     // inputs
                     if let Some(ev) = Event::parse_relevant(event)
                     {
                         self.handle_event(ev);
                     } 
                     if self.exit
                     {
                         *control_flow = ControlFlow::Exit
                     }

                     // game logic
                    (self.game_logic)(&mut self);


                     // render
                     now = std::time::Instant::now();
                     if render_date < now
                     {
                         let delta = (now-render_date+delay).as_nanos();
                         //println!("{} fps ({} ns)", 1_000_000_000/(delta+1), delta);
                         self.render();
                         render_date = now + delay;
                     }

                 });
    }
    
}

fn game_logic(game: &mut Game)
{

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);
    let sensibility = 0.005;
    let speed = 0.001; // parce que pourquoi pas.

    let (mouse_x, mouse_y) = game.devices.mouse_motion();
    camera_rot[1] -= (mouse_x as f32) * sensibility;
    camera_rot[0] -= (mouse_y as f32) * sensibility;

    if game.devices.key_pressed(Key::Q) {
        camera_pos[2] = camera_pos[2] - speed;
    }
    if game.devices.key_pressed(Key::D) {
        camera_pos[2] = camera_pos[2] + speed;
    }
    if game.devices.key_pressed(Key::Z) {
        camera_pos[0] = camera_pos[0] + speed;
    }
    if game.devices.key_pressed(Key::S) {
        camera_pos[0] = camera_pos[0] - speed;
    }
    if game.devices.key_pressed(Key::Escape) {
        game.exit = true;
    }
    game.graphic_engine.camera.relative_move(camera_pos);
    game.graphic_engine.camera.rotation(camera_rot.clone());

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


// the holder outlives the scene
fn make_scene(
    disp: &Display,
    holder: & mut RessourcesHolder,
    base: &Base
) -> Result<Scene, EngineError>
{
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "textured_cube.obj", &ressources_path)?;
    holder.load_wavefront(disp, "reds.obj", &ressources_path)?;
    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(disp, "teto.obj", &ressources_path)?;
    holder.load_wavefront(disp, "terrain.obj", &ressources_path)?;


    let _sphere_mauve = holder.get_object("transparent_sphere", "Sphere").unwrap();
    let teto = holder
        .get_object("teto", "Lat式改変テト_mesh_Lat式改変テト")
        .unwrap();
    let red = holder.get_object("reds", "Cube_translaté_Cube.002").unwrap();
    let zeldo = holder.get_object("textured_cube", "Cube.001").unwrap();
    let map_elements = holder.get_whole_content("terrain").unwrap();
    // le buffer d'instanciation pour la map
    let map_position = vec![Similarity {
        world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
    }];
   

    holder.add_parameters(Params::new().polygon_line(), "wireframe");

    let red = holder.obj_parameters(red, "wireframe")?;
    
    holder.add_tile(&disp, &base, "edgytet.png")?;
    
    let tile = holder.get_tile("edgytet", &disp)?;

    let tile_position = vec![Similarity {
        world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
    }];


	
    
    // le buffer d'instanciation pour les cubes
    let instances = (0..30).map(|_| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random::<f32>(), rand::random::<f32>()), 
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>();

    
    let mut scene = Scene::new();

    scene.add(vec![red, zeldo, teto], instances);
    scene.add(vec![map_elements], map_position);

    scene.add(vec![commode], position_commode);

    Ok(scene)
}



fn main() -> Result<(), EngineError>
{
    
    let event_loop = EventLoop::new();
    let game = Game::new(game_logic, &event_loop);
    game.run(event_loop)

}


