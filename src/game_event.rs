

pub enum GameEvent
{
    QuitRequested,
    Pop(usize),
    Push(String),
    PlaySound(String,Option<[f32; 3]>),
    PlaySound_timeLimit(String,Option<f32>,Option<[f32; 3]>)
}
