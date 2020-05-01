use sounds::music::*;

fn main()
{
    let mut global_music = OneSound:: new("BMTH-Blessed_With_A_Curse.wav");
    global_music.play_all();
}
