#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};

//use glutin::event_loop::EventLoop;

mod processing;
mod engine;

use processing::*;
use engine::*;

mod misc;
use misc::*;

        

use nalgebra::base::*;

use rayon::prelude::*;
use rayon::iter::*;



use std::path::Path;

fn matrix_to_array(mat: Matrix4<f32>) -> [[f32; 4]; 4]
{
	let mut out = [[0.; 4]; 4];
	for i in 0..4
	{
		for j in 0..4
		{
			out[j][i] = *mat.get(i + 4*j).unwrap();
		}
	}
	out

}










fn main() {

    let args: Vec<String> = std::env::args().collect();
    let executable_path = Path::new(&args[0]);
    let crate_path = executable_path.parent().unwrap().parent().unwrap().parent().unwrap();
    let ressources_path = crate_path.join(Path::new("ressources"));
    // The first argument is the path that was used to call the program.
    println!("My path is {:?}.", executable_path);
    println!("Crate path is {:?}.", crate_path);


    let mut event_loop = glutin::EventsLoop::new();
    
    let mut graphics = Graphical::new(&event_loop);
    let mut holden = ModelsHolder::new();

    holden.load_wavefront(&graphics,
                          "textured_cube.obj",
                          &ressources_path);
    holden.load_wavefront(&graphics,
                          "reds.obj",
                          &ressources_path);
    holden.load_wavefront(&graphics,
                          "transparent_sphere.obj",
                          &ressources_path);
    holden.load_wavefront(&graphics,
                          "teto.obj",
                          &ressources_path);
    holden.load_wavefront(&graphics,
                          "terrain.obj",
                          &ressources_path);
    
    
    
    // list of teapots with position and direction
    let mut teapots = (0 .. 30)
        .map(|_| {
            let pos: (f32, f32, f32) = ((rand::random::<f32>()), rand::random::<f32>(), rand::random::<f32>());
	    let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
            let rot: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
	    let rot = (rot.0*6., rot.1*6., rot.2*6.);
            let size: f32 = rand::random();
            (pos, rot, size)
	    
        })
        .collect::<Vec<_>>();

    
    // building the vertex buffer with the attributes per instance
    // contient les positions des objets instanciés
    let mut per_instance = {


        // créé un vecteur de 10000 vertex (un point par object)
        let data = teapots.iter().map(|_| {
            Attr
            {
                world_transformation: [[0.; 4]; 4],
            }
        }).collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&graphics.display, &data).unwrap()
    };

    

    let map_position = glium::vertex::VertexBuffer::dynamic(
        &graphics.display,
        &vec![Attr{world_transformation:[[1.0, 0.0, 0.0, 0.0],
                                         [0.0, 1.0, 0.0, 0.0],
                                         [0.0, 0.0, 1.0, 0.0],
                                         [0.0, -1.0, 0.0, 1.0]]}]).unwrap();
    
    struct ToDisp<'a>
    {
        vertex_buffer: &'a glium::vertex::VertexBufferAny,
        material: Option<&'a processing::material::Material>
    }

    graphics.camera.set_position((0., 0., 0.));
    println!("\n HOLDER: {:?} \n", holden);
    let sphere_mauve = holden.get("transparent_sphere", "Sphere").unwrap();
    let teto = holden.get("teto", "Lat式改変テト_mesh_Lat式改変テト").unwrap();
    let red = holden.get("reds", "Cube_translaté_Cube.002").unwrap();
    let zeldo = holden.get("textured_cube", "Cube.001").unwrap();
    let map_elements = holden.get_whole_file("terrain").unwrap();

    {//variable locale aux crochets
        let mut mapping = per_instance.map();
        for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
		let rot = Matrix4::new_rotation(Vector3::new((src.1).0, (src.1).1, (src.1).2));
		let translation = Matrix4::new(
			1.,0.,0.,(src.0).0,
			0.,1.,0.,(src.0).1,
			0.,0.,1.,(src.0).2,
			0.,0.,0.,   1.    );
		let aggr=src.2/1000.;
		let aggrandissement = Matrix4::new(
		    aggr,0.,0.,0.,
		    0.,aggr,0.,0.,
		    0.,0.,aggr,0.,
		    0.,0.,0.,1. );
            
	let transfs =  translation*rot*aggrandissement;
            dest.world_transformation = matrix_to_array(transfs);
        }
    }

    use glutin::Event::DeviceEvent;
    use glutin::KeyboardInput;
    use glutin::VirtualKeyCode;

    use glutin::ControlFlow;
    
//    let event_loop = graphics.get_event_loop();


    let mut camera_pos = (0., 0., 0.);
    let mut camera_rot = (0., 0., 0.);

    let mut x = 0.;

    use std::collections::HashSet;
    use std::hash::Hash;
    let mut keys = HashSet::new();
    let sensibility = 0.0005;
    
    event_loop.run_forever( |event|
                             {

                                 println!("KEYS EVENTS {:?}", keys);
                                 
                                 graphics.camera.relative_move(camera_pos);

                                 graphics.camera.rotation(camera_rot.clone());
                                 camera_pos = (0., 0., 0.);

                                 //        println!("CAMERA IS AT {:?}", camera_pos.clone());

                                 let mut frame = graphics.frame();
                                 graphics.update_dimensions();
                                 frame.clear();

                                 //        frame.draw(&graphics, &teto, &per_instance);
                                 //        frame.draw(&graphics, &sphere_mauve, &per_instance);
                                 //    frame.draw(&graphics, &map, &map_position);
                                 frame.draw(&graphics, &red, &per_instance);
                                 frame.draw(&graphics, &zeldo, &per_instance);

                                 map_elements
                                     .iter()
                                     .for_each(
                                         |ob|
                                         {
                                             frame.draw(&graphics, &ob, &map_position);
                                         });
                                 
                                 frame.show();

                                 for keycode in keys.iter(){
                                     match keycode
                                     {
                                         VirtualKeyCode::Z =>
                                         {
                                             camera_pos.0 = camera_pos.0 + 1.;
                                         },
                                         VirtualKeyCode::S =>
                                         {
                                             camera_pos.0 = camera_pos.0 - 1.;
                                         },
                                         VirtualKeyCode::Q =>
                                         {
                                             camera_pos.2 = camera_pos.2 - 1.;
                                         },
                                         VirtualKeyCode::D =>
                                         {
                                             camera_pos.2 = camera_pos.2 + 1.;
                                         },
                                         _ =>
                                         {
                                             

                                         },
                                         
                                     };
                                 }

                             
                                 match event
                                   {
                                       glutin::Event::WindowEvent { event, .. } =>
                                           match event {
                                               glutin::WindowEvent::CloseRequested => ControlFlow::Break,
                                               _ => ControlFlow::Continue,
                                           },
                                       glutin::Event::DeviceEvent {event, ..}=>
                                       {

                                           match event
                                           {
                                               glutin::DeviceEvent::Key(keyboard_input) =>
                                               {
                                                   match keyboard_input
                                                       .virtual_keycode
                                                   {
                                                       None => ControlFlow::Continue,
                                                       Some(keycode) =>
                                                       {
//                                                           println!("KEY: {:?} keyboard_input: {:?}", keycode, keyboard_input);
                                                           if keyboard_input.state == glutin::ElementState::Released
                                                           {
                                                               keys.remove(&keycode);
                                                           }
                                                           else
                                                           {
                                                               keys.insert(keycode);
                                                           };
                                                           ControlFlow::Continue
   
                                                       }

                                                   }
                                               },
                                               
                                               glutin::DeviceEvent::Motion{axis, value} =>
                                               {
                                                   println!("MOTION Axe:{} value:{}", axis, value);
                                                   match axis
                                                   {
                                                       0 =>
                                                       {
                                                           camera_rot.1 += (value as f32)*sensibility;
                                                       },
                                                       1 =>
                                                       {
                                                           camera_rot.0 += (value as f32)*sensibility;

                                                       },
                                                       _ =>
                                                       {
    //                                                       println!("unknown axis");
                                                       }
                                                   }
                                                   ControlFlow::Continue
                                               },
                                               _ =>
                                               {
      //                                             println!("DEVICE NONKEYBOARD EVENT: {:?}", event);
                                                   ControlFlow::Continue
                                               }
                                           }
                                       },
                                       _ =>
                                       {
        //                                   println!("OTHER EVENT: {:?}", event);
                                           ControlFlow::Continue
                                       }
                                       
                                   }
                                   
                                   
                               }

    );

    println!("FINALLY: {:?}", camera_pos);

        // the main loop
        
        //graphics.camera.rotation((0., 0.01, 0.001));



    
}




