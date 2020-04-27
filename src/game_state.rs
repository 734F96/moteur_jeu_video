use super::{Game, GameEvent};
use graphics::{Scene, Graphical, Frame};
use events_handling::DevicesState;
use graphics::glium::glutin::event_loop::EventLoopProxy;

pub struct GameState
{
    pub scene: Scene,
    pub logic: fn(&mut GameState, &DevicesState),
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,
    proxy: EventLoopProxy<GameEvent>
}



impl GameState
{
    pub fn new(scene: Scene,
               logic: fn(&mut GameState, &DevicesState),
               render_behavior: RenderBehavior,
               logic_behavior: LogicBehavior,
               proxy: EventLoopProxy<GameEvent>) -> Self
    {
        Self
        {
            scene: scene,
            logic: logic,
            render_behavior: render_behavior,
            logic_behavior: logic_behavior,
            proxy: proxy
        }
    }

    pub fn send_event(&self, user_event: GameEvent)
    {
        self.proxy.send_event(user_event);
    }
}

#[derive(Debug, PartialEq)]
pub enum RenderBehavior
{
    NoRender,
    Superpose,
    Blocking
}

#[derive(Debug, PartialEq)]
pub enum LogicBehavior
{
    Superpose,
    Blocking
}

pub struct GameStateStack(Vec<GameState>);

impl GameStateStack
{
    pub fn new() -> Self
    {
        Self(Vec::new())
    }

    pub fn push(&mut self, state: GameState)
    {
        self.0.push(state);
    }

    pub fn pop(&mut self)
    {
        self.0.pop();
    }

    pub fn iter(&self) -> std::slice::Iter<GameState>
    {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<GameState>
    {
        self.0.iter_mut()
    }
    pub fn render(&mut self, gr: &Graphical, frame: &mut Frame)
    {
        let first_block = self.iter()
            .rposition(|state| state.render_behavior == RenderBehavior::Blocking);
        let to_skip = match first_block
        {
            None => 0,
            Some(pos) => pos
        };
        for state in self.iter_mut().skip(to_skip)
            .filter(|state| state.render_behavior != RenderBehavior::NoRender)
        {
            state.scene.render(gr, frame)
        }
    }
    pub fn logic(&mut self, devices: &DevicesState)
    {
        let first_block = self.iter()
            .rposition(|state| state.logic_behavior == LogicBehavior::Blocking);
        let to_skip = match first_block
        {
            None => 0,
            Some(pos) => pos
        };
        for state in self.iter_mut().skip(to_skip)
        {
            (state.logic)(state, devices);
        }
    }
}
