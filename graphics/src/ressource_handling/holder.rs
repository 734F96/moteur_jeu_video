use std::collections::HashMap;
use std::path::Path;

use super::{Wavefront, Group};
use crate::engine::Display;
use base::EngineError;


/**
A holder is a struct mapping somme ressources to their names
Like a hashmap, but we can implement our own methods on it.
*/
#[derive(Debug)]
pub struct Holder<T> (HashMap<String, T>);

impl<T> Holder<T>
{
    /// Creates a new Holder.
    pub fn new() -> Self
    {
        Self(HashMap::new())
    }

    /// Returns a reference to the eventual value corresponding to the given key.
    pub fn get(&self, s: &str) -> Option<&T>
    {
        self.0.get(s)
    }

    /// Inserts an entry.
    pub fn insert(&mut self, s: String, el: T)
    {
        self.0.insert(s, el);
    }
    
    
}

/// Thoses methods are specific to wavefront holders
impl Holder<Wavefront>
{
    /**
    Fetch the vertex and material data of a 3D object corresponding to the given key (which is in this case the name of the object).
     */
    pub fn get_object(
        &self,
        file: &str,
        model_name: &str,
    ) -> Result<Vec<Group>, EngineError> {
        match self.get(file) {
            Some(wavefront) => wavefront.get_object(model_name.to_string()),
            None => EngineError::new(&format!("file {} doesn't exist!", file)),
        }
    }

    /**
    Like get_objet, but returns the union of all the objects of the file.
     */
    pub fn get_whole_content(
        &self,
        file: &str,
    ) -> Result<Vec<Group>, EngineError> {
        match self.get(file) {
            None => EngineError::new("file doesn't exist!"),
            Some(wavefront) => Ok({
                wavefront
                    .objects
                    .keys()
                    .map(|obj_name| self.get_object(file, obj_name).unwrap())
                    .flatten()
                    .collect::<Vec<_>>()
            }),
        }
    }

    /**
    Tries to import a wavefront.
     */
    pub fn load_wavefront(
        &mut self,
        disp: &Display,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), EngineError> {
        let path_to_wavefront = Path::new(filename);
        let stem = match path_to_wavefront.file_stem() {
            Some(st) => st,
            _ => return EngineError::new("cannot parse stem, is the address empty?"),
        };
        let stem = match stem.to_str() {
            Some(st) => st,
            _ => return EngineError::new("cannot parse stem, is the address empty?"),
        };
        let stem = stem.to_string();
        let path_to_mtl = path_to_wavefront.with_extension("mtl");

        println!("stem: {:?}", stem);
        println!("mtl: {:?}", path_to_mtl);

        self.insert(
            stem.clone(),
            Wavefront::new(disp, path_to_wavefront, &path_to_mtl, ressources_path)?,
        );
        Ok(())
    }

    /**
    Remove a file from the graphical memory.
    (bad idea for now)
     */ 
    pub fn unload(&mut self, filename: &str) -> Result<(), EngineError> {
        if self.0.contains_key(filename) {
            self.0.remove(filename);
            return Ok(());
        } else {
            return EngineError::new("file not found");
        }
    }
   
}

