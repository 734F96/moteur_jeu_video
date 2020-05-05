use ears::Sound;
use ears::AudioController;
use ears::SoundData;
use std::time::Instant;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use base::EngineError;

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




   pub fn new_from_data(sound_data: SoundRessource) -> Result<Self, EngineError>
   {
        let sound = Sound::new_with_data(sound_data.data)?;
        Ok(Self{music: sound})
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

  pub fn give_position(&mut self,position: [f32; 3])
  {
    self.music.set_position(position)
  }


}

impl fmt::Debug for OneSound {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      
	write!(f, "OneSound")
    }
}


pub struct SoundRessource
{
   pub data : Rc<RefCell<SoundData>>
} 


impl SoundRessource{

    pub fn new(path: &str) -> Self
    {
        Self{ data : Rc::new(RefCell::new(SoundData::new(path).unwrap()))}
             
    }

}

impl fmt::Debug for SoundRessource {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      
	write!(f, "SoundRessource")
    }
}


