use ears::Sound;
use ears::AudioController;
use std::time::Instant;

/**
*ears use internally libsndfile so the format of sound ressources format must to match libsndfile accepted formats.
* including: WAV, FLAC, PAF
*
**/

pub struct OneSound
{
   music: Sound

}

impl OneSound
{
    pub fn new(path_given : &str) -> Self 
        { 
		Self { music : Sound::new(path_given).unwrap()} 
         
	}

    pub fn play_nolimit(&mut self) 
    {
        while(true)
	{self.music.play();
	  while (self.music.is_playing())
		{}
	}
    }

    pub fn play_all(&mut self) 
    {
	self.music.play();
        while (self.music.is_playing())
	{}
	
    }

   pub fn play_time_limit (&mut self,time: f32)
   {
	let start=Instant::now();
	self.music.play();
	while( start.elapsed().as_secs() as f32 != time)
	{	 if (self.music.is_playing()) {}
		else {self.music.play()}
	}
   }

  pub fn set_position(&mut self,position: [f32; 3])
  {
    self.set_position(position)
  }


}
