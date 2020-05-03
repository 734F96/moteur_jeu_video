use std::path::Path;
use glium::vertex::VertexBuffer;
use std::sync::Arc;
use std::path::PathBuf;

use super::{Material, Object, Wavefront, Holder, Tile, Group, Vertex, Register, Handle};
use crate::engine::{ProgramId, Display, Params};

use base::{Base, EngineError};


/// Registers all the ressources.

#[derive(Debug)]
pub struct RessourcesHolder {
    /// The meshes and materials data.
    wavefronts: Holder<Wavefront>,
    /// The existing shaders programs
    programs: Holder<ProgramId>,
    /// The 2d tiles
    tiles: Holder<Tile>,
    /// The drawing parameters
    params: Holder<Arc<Params>>, // overkill

    /// All the renderable objects constructed from the other ressources
    objects_register: Register<Object>
}

impl RessourcesHolder {
    /// Create a new RessourcesHolder
    pub fn new() -> Self {
        Self
        {
            wavefronts: Holder::new(),
            programs: Holder::new(),
            tiles: Holder::new(),
            params: Holder::new(),
	    objects_register: Register::new(),
        }.init()
    }

    fn store_object(&mut self, obj: Object) -> Handle<Object>
    {
	self.objects_register.add(obj)
    }

    /// we suppose that the data exists if the handle does
    pub fn get_by_handle(&self, handle: Handle<Object>) -> &Object
    {
	self.objects_register.get(handle)
    }
    
    pub fn free_object(&mut self, handle: Handle<Object>)
    {
	self.objects_register.remove(handle);
    }
    
    /// used to insert some default values.
    fn init(mut self) -> Self
    {
        self.add_parameters(Params::new(), "scene");
        self.add_parameters(Params::new().always_top(), "foreground");
        self
    }
    
    /// Adds a shader program    
    pub fn register_program(&mut self, pgrm: ProgramId, name: String)
    {
        self.programs.insert(name, pgrm);
    }

    /// Fetches a shader program
    pub fn get_program(&self, name: &str) -> Option<&ProgramId>
    {
        self.programs.get(name)
    }
    
    /// Returns a default shader program capable of handling the given material.
    fn associate_program(&self, mat: &Material) -> ProgramId
    { 
        match mat
        {
            &Material::Default =>
                *self.programs.get("default").unwrap(),
            &Material::Textured { .. } =>
            {
                *self.programs.get("textured").unwrap()
            }
            &Material::NonTextured { .. } =>
            {
                *self.programs.get("nontextured").unwrap()            
            }
        }
    }

    /// Adds some drawing parameters
    pub fn add_parameters(&mut self, params: Params, name: &str)
    {
        self.params.insert(name.to_string(), Arc::new(params))
    }

    /// Add a tile based on the given image.
    pub fn add_tile( &mut self,
                      display: &Display,
                      base: &Base,
                      image_path: &str ) -> Result<(), EngineError>
    {
        let path = PathBuf::from(image_path);
        let name = match path.file_stem()
        {
            None => EngineError::new("tile texture has invalid name"),
            Some(stem) =>
                match stem.to_str()
                .map(|s| s.to_string())
            {
                None => EngineError::new("tile texture has invalid name"),
                Some(thing) => Ok(thing)
            }
        }?;
        let tile = Tile::new(base, display, path)?;
        self.tiles.insert(name, tile);
        Ok(())
    }

    /// Returns an Object using the given drawing parameters.
    pub fn obj_parameters(&mut self, obj: Object, params_name: &str) -> Result<Handle<Object>, EngineError>
    {
        let obj = match self.params.get(params_name)
        {
            None => EngineError::new("unknown parameter"),
            Some(params) => Ok(Object::new(obj.data, params.clone()))
        }?;
	Ok(self.store_object(obj))
    }
    
    
    /// Fetch a displayable tile as a drawable Object
    pub fn get_tile(&mut self,
                    name: &str,
                    display: &Display ) -> Result<Handle<Object>, EngineError>
    {
        match self.tiles.get(name)
        {
            None => EngineError::new("Tile inexistant!"),
            Some(tile) =>
            {
                let program = *self.get_program("textured_2d").unwrap();
                let (w, h) = tile.dims;

	        let mesh = vec![
	            Vertex{position: [0., 0., 0.], texture: [0., 0.], .. Default::default()},
	            Vertex{position: [w, 0., 0.], texture: [1., 0.], .. Default::default()},
	            Vertex{position: [w, h, 0.], texture: [1., 1.], .. Default::default()},
	            Vertex{position: [0., 0., 0.], texture: [0., 0.], .. Default::default()},
	            Vertex{position: [0., h, 0.], texture: [0., 1.], .. Default::default()},
	            Vertex{position: [w, h, 0.], texture: [1., 1.], .. Default::default()},
	        ];

                let vbo = VertexBuffer::new(&display.display, &mesh).unwrap();

                let group = Group
                {
                    vertexes: Arc::new(vbo),
                    material: tile.texture.clone()
                };
                let params = self.params.get("foreground").unwrap().clone();
                Ok(self.store_object(Object::new(vec![(group, program)], params)))
            }
        }
    }
   
    /// Fetch a displayable 3D object from a wavefront file name and the name of the requested object.
    pub fn get_object(&mut self, file: &str, model_name: &str) -> Result<Handle<Object>, EngineError> {
        let shape_data = self.wavefronts.get_object(file, model_name);
        let params = self.params.get("scene").unwrap().clone();
        let whole_data = shape_data?
            .iter()
            .map(|group| {
                let mat: &Material = &group.material;
                let program = self.associate_program(mat);
                (
                    group.clone(),
                    program,
                )
            })
            .collect::<Vec<_>>();
        Ok(self.store_object(Object::new(whole_data, params)))
    }

    /// Like get_object, but the object fetched is the union of all the objects of the wavefront file.
    pub fn get_whole_content(&mut self, file: &str) -> Result<Handle<Object>, EngineError> {
        let shape_data = self.wavefronts.get_whole_content(file);
        let params = self.params.get("scene").unwrap().clone();
        let whole_data = shape_data?
            .iter()
            .map(|group| {
                let mat: &Material = &group.material;
                let program = self.associate_program(mat);
                (
                    group.clone(),
                    program,
                )
            })
            .collect::<Vec<_>>();
        Ok(self.store_object(Object::new(whole_data, params)))
    }

    /// Loads a wavefront file
    pub fn load_wavefront(
        &mut self,
        disp: &Display,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), EngineError> {
        self.wavefronts
            .load_wavefront(disp, filename, ressources_path)
    }

    /// Unloads a wavefront file.
    pub fn unload(&mut self, filename: &str) -> Result<(), EngineError> {
        self.wavefronts.unload(filename)
    }
}
