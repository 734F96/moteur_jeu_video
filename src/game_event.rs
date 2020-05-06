
#[derive(Debug)]
pub enum GameEvent
{
    QuitRequested,
    Pop(usize),
    Push(String),
    PlaySound(String,Option<[f32; 3]>),

    PlaySoundTimeLimit(String,Option<f32>,Option<[f32; 3]>),
    DownVolume(),
    UpVolume()

}
