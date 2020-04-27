#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true)
    }
}

pub extern crate glium;

pub mod engine;
pub mod misc;
pub mod ressource_handling;

pub use engine::*;
pub use ressource_handling::*;
pub use misc::*;
pub use nalgebra;
pub use nalgebra_glm;
