use std::io::Read ;
use std::string::String;
use std::fs::File;
use std::path::{Path, PathBuf};

use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4, Vec3};
use nalgebra::base::Matrix4;
use glium::implement_vertex;
use glium::vertex::VertexBuffer;

use crate::engine::Display;

/// used for debug, will be discarded eventually.
pub fn maybe<T>(option: Option<T>, s: &'static str) -> T
{
    assert!(option.is_some(), s);
    option.unwrap()
}

/// Normalize a vector. Will be eventually be discarted.
pub fn normalize_vec(v: (f32, f32, f32)) -> (f32, f32, f32)
{
    let norm = (v.0*v.0 + v.1*v.1 + v.2*v.2).sqrt();
    (v.0/norm, v.1/norm, v.2/norm)
}

/// Some vector product. Will eventually be discarted
pub fn v_prod(u: (f32, f32, f32), v: (f32, f32, f32)) -> (f32, f32, f32)
{
    (
        u.1*v.2 - u.2*v.1,
        -u.0*v.2 + u.2*v.0,
        u.0*v.1 - u.1*v.0
    )
}

/// Reads a file and returns its content
pub fn read_file(file_name: PathBuf) -> String
{
    println!("OPENING {:?}", file_name);
	let mut f = match File::open(file_name)
	{
		Ok(fichier) => fichier,
		err => panic!(format!("Couldn't open file at {:?}", err))
	};
	let mut buffer = String::new();
	match f.read_to_string(&mut buffer)
	{
		Ok(_) => (),
		_ => panic!("Couldn't read file")
	};

	buffer
}



// TODO: faire une meilleure gestion des erreures
/// Finds the path of the ressources folder
pub fn get_ressources_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    //the only relevant path we can get is the executable's since the execution dir could be anywhere
    let executable_path = Path::new(&args[0]);
    for path in executable_path.ancestors()
    {
        let ressources_path = path.join(Path::new("ressources"));
        if ressources_path.exists()
        {
            return ressources_path
        }
    }
    panic!("Can't find any 'ressources' folder while going up the path");
}


/// Sometimes we need an array and not a matrix
pub fn matrix_to_array(mat: Matrix4<f32>) -> [[f32; 4]; 4] {
    let mut out = [[0.; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            out[j][i] = *mat.get(i + 4 * j).unwrap(); // guaranteed
        }
    }
    out
}



/**
The matrix of a similarity.
Used to displace, rotate and resize a 3D object.
https://en.wikipedia.org/wiki/Similarity_(geometry)
*/
#[derive(Copy, Clone, Debug)]
pub struct Similarity {
    pub world_transformation: [[f32; 4]; 4],
}
implement_vertex!(Similarity, world_transformation);

impl Similarity
{
    /// creates a new Similarity from a position, a vector of 3 angles and a scale
    pub fn new(pos: Vec3, rot: Vec3, scale: f32) -> Self
    {
        let rot =
        rotation(rot.x, &vec3(1., 0., 0.)) *
        rotation(rot.y, &vec3(0., 1., 0.)) *
        rotation(rot.z, &vec3(0., 0., 1.));
        let trans = translation(&vec3(pos.x, pos.y, pos.z));
        let resize = TMat4::from_diagonal(&vec4(scale, scale, scale, 1.));
        Self
        {
            world_transformation: *(trans*rot*resize).as_ref()
        }
    }

    /// returns the pos, rot and scale encoded within the inner matrix
    pub fn deconstruct(&self) -> (Vec3, Vec3, f32)
    {
        let origin = vec4(0., 0., 0., 1.);
        let x = vec4(1., 0., 0., 0.);
        let y = vec4(0., 1., 0., 0.);
        let z = vec4(0., 0., 1., 0.);
        let it = self.world_transformation.iter()
            .flatten().cloned();
        let mat: TMat4<f32> = TMat4::from_iterator(it);
        let scale = (mat*x).norm();
        let trans = mat*origin;
        let a_x = x.angle(&(mat*x));
        let a_y = y.angle(&(mat*y));
        let a_z = z.angle(&(mat*z));
        let rot = vec3(a_x, a_y, a_z);
        (trans.xyz(), rot, scale)
    }

    /// creates a new Similarity from a position, a vector of 3 angles and a scale
    pub fn set_pos(mut self, pos: Vec3, rot: Vec3, scale: f32)
    {
        let rot =
        rotation(rot.x, &vec3(1., 0., 0.)) *
        rotation(rot.y, &vec3(0., 1., 0.)) *
        rotation(rot.z, &vec3(0., 0., 1.));
        let trans = translation(&vec3(pos.x, pos.y, pos.z));
        let resize = TMat4::from_diagonal(&vec4(scale, scale, scale, 1.));
        self.world_transformation = *(trans*rot*resize).as_ref()
    }
}



/// creates a new vertexbuffer of world positions
pub fn new_vertexbuffer<T: Copy + glium::Vertex>(disp: &Display, positions: &Vec<T>) -> VertexBuffer<T>
{
    VertexBuffer::dynamic(
        &disp.display,
        positions,
    ).unwrap()

}

