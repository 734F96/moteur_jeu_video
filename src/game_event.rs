use sounds::{SoundRessource};


pub enum GameEvent
{
    QuitRequested,
    Pop(usize),
    Push(String),
    PlaySound(SoundRessource,String,Option<[f32; 3]>),
    PlaySound_timeLimit(SoundRessource,String,Option<f32>,Option<[f32; 3]>)
}
