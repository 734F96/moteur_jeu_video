use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use glium::texture::{RawImage2d, Texture2d};
use glium::vertex::{VertexBuffer};
use obj::{Mtl, Obj};
use genmesh::{Polygon, Quad, Triangle};

use super::{Vertex, Group, Material};
use crate::engine::Display;
use base::EngineError;


/**
This structure represents a set of 3D objects and their shared materials.
It typicaly contains all the information a wavefront file and their associated mtl file can provide.
 */
#[derive(Debug)]
pub struct Wavefront {
    pub objects: HashMap<String, Vec<Group>>,
    pub materials: HashMap<String, Arc<Material>>,
}


impl Wavefront {
    /**
    Imports new wavefront file.
     */
    pub fn new(
        disp: &Display,
        path_to_wavefront: &Path,
        path_to_mtl: &Path,
        ressources_path: &Path,
    ) -> Result<Self, EngineError> {


	// try to load the files
        let path_to_wavefront = ressources_path.join(path_to_wavefront);
        let path_to_mtl = ressources_path.join(path_to_mtl);
        let file = File::open(path_to_wavefront)?;
        let mut bufreader = ::std::io::BufReader::new(file);
        let obj = Obj::load_buf(&mut bufreader)?;
        let file = File::open(path_to_mtl)?;
        let mut bufreader = ::std::io::BufReader::new(file);
        let mtl = Mtl::load(&mut bufreader);




        /*
        parsing the materials from the .mtl
        several kind of material are supported for now: Textured, NonTextured
        and Default (constructed according to what is being found in the mtl)
        */
        let mut materials = HashMap::new();

        for material in mtl.materials.iter() {
            let mat = match &material {
                obj::Material {
                    ks: Some(specular),
                    ns: Some(specular_exponent),
                    tr: transparency,
                    d: opacityy,
                    map_kd: Some(texture_path),
                    ..
                } => {
                    let opacity = opacityy.unwrap_or(1.)
			.min(1. - transparency.unwrap_or(0.));
                    let image = image::open
			(
			    ressources_path.join(Path::new(texture_path))
			)?.to_rgba();

                    let image_dimensions = image.dimensions();
                    let image =
                        RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                    let texture = Texture2d::new(&disp.display, image)?;
                    Material::Textured {
                        texture: texture,
                        specular_color: *specular,
                        specular_exponent: *specular_exponent,
                        opacity: opacity,
                    }
                }
                obj::Material {
                    ka: Some(ambiant),
                    kd: Some(diffuse),
                    ks: Some(specular),
                    ke: Some(emission),
                    ns: Some(specular_exponent),
                    tr: transparency,
                    d: opacity,
                    map_kd: None,
                    ..
                } => {
                    let opacity = opacity.unwrap_or(1.).min(1. - transparency.unwrap_or(0.));
                    
                    Material::NonTextured {
                        ambiant_color: *ambiant,
                        diffuse_color: *diffuse,
                        specular_color: *specular,
                        specular_exponent: *specular_exponent,
                        emission_color: *emission,
                        opacity: opacity,
                    }
                }

                _ => Material::Default,
            };
            materials.insert(material.name.clone(), Arc::new(mat));
        }

        // starting to parse the .obj and to construct the Wavefront structure
        let mut objects = Wavefront {
            objects: HashMap::new(),
            materials: materials,
        };

        for object in obj.objects.iter() {
            let mut groups = Vec::new();
            for group in object.groups.iter() {
                let mut mesh = Vec::new();

                for polygon in group.polys.iter() {
                    // polygons may be triangles or quads
                    match polygon {
                        &Polygon::PolyTri(Triangle {
                            x: v1,
                            y: v2,
                            z: v3,
                        }) => {
                            for v in [v1, v2, v3].iter() {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex {
                                    position: position,
                                    normal: normal,
                                    texture: texture,
                                })
                            }
                        }
                        &Polygon::PolyQuad(Quad {
                            x: v1,
                            y: v2,
                            z: v3,
                            w: v4,
                        }) => {
                            for v in [v1, v2, v3].iter() {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex {
                                    position: position,
                                    normal: normal,
                                    texture: texture,
                                })
                            }
                            for v in [v3, v4, v1].iter() {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex {
                                    position: position,
                                    normal: normal,
                                    texture: texture,
                                })
                            }
                        }
                    }
                }

                let default_material = Arc::new(Material::Default);
                
                // adding the group of voxel just constated and the name of its associed material
                // this is fine since one is not supposed to access the material very often
                let mat = group.material.as_ref()
                    .map(|mat| mat.name.clone())
                    .map(|name| objects.materials.get(&name).unwrap_or(&default_material))
                    .unwrap_or(&default_material).clone();
                groups.push(Group {
                    vertexes: Arc::new(
                        VertexBuffer::new(&disp.display, &mesh)
                            .unwrap()),
                    material: mat,
                    
                });
            }
            // the object is finished and added to the Wavefront structure
            objects
                .objects
                .insert(object.name.clone(),
                        groups);
        }

        Ok(objects)
    }

    /**
    Fetches the material and mesh data corresponding to the given object name.
     */
    pub fn get_object(&self, name: String) -> Result<Vec<Group>, EngineError>
    {

        match self.objects.get(&name)
        {
            Some(thing) => Ok(thing.to_vec()),
            None => EngineError::new(&format!("object '{}' does not exist", name))
        }
    }
}
