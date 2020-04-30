mod game;
mod game_state;
mod game_event;
mod components;

pub use game::*;
pub use game_state::*;
pub use game_event::*;
pub use components::*;

/// The elements needed to create a game
pub mod prelude
{
    pub use crate::{
	Game,
	GameState,
	RenderBehavior,
	LogicBehavior,
	GameEvent
    };
    pub use ::base::EngineError;
    pub use ::graphics::{
	glium::glutin::event_loop::EventLoopProxy,
	nalgebra::Vector3,
	nalgebra_glm::{TMat4, vec3, vec4, translation, rotation},
	Similarity,
	get_ressources_path,
	Scene,
	Params,
	Light
    };
    pub use ::events_handling::{Key, DevicesState};
    pub use imgui::{Window, im_str, Condition, Ui};

}
