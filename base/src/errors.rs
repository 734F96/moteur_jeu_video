use std::io;

#[derive(Debug)]
pub struct EngineError(String);

impl EngineError
{
    pub fn new<T>(msg: &str) -> Result<T, Self>
    {
        Err(Self(String::from(msg)))
    }
}


macro_rules! engine_error_from {
    ($type:path) => {
        impl From<$type> for EngineError {
            fn from(error: $type) -> Self
            {
                Self(format!("{}", error))
            }
        }
    };
    (&$type:path) => {
        impl From<&$type> for EngineError {
            fn from(error: &$type) -> Self
            {
                Self(format!("{}", error))
            }
        }
    };
}

engine_error_from!(io::Error);
engine_error_from!(image::error::ImageError);
engine_error_from!(glium::texture::TextureCreationError);
engine_error_from!(glium::ProgramCreationError);
engine_error_from!(&str);
engine_error_from!(String);
