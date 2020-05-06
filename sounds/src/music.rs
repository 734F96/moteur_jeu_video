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
   pub music: Sound,
   pub start : Instant,
   pub end :  f32,

}


// end == -1 -> play all the sound 
// end >0 -> play the sound during end time
// end == -2  -> repeat the sound 


impl OneSound
{
    pub fn new(path_given : &str) -> Self 
    { 
	Self { music : Sound::new(path_given).unwrap(),
               start : Instant::now(),
               end : -1.,  
             } 
    }

   pub fn new_from_data(sound_data: SoundRessource) -> Result<Self, EngineError>
   {
        let sound = Sound::new_with_data(sound_data.data)?;
        Ok(Self{music: sound, start : Instant::now(), end : -1.,})
   }
   
   pub fn set_end(&mut self,duration: f32)
   {
     self.end=duration;
   }


    pub fn play_all(&mut self) 
    {
	self.music.play();
        self.start=Instant::now();
	
    }
  
   pub fn is_playing(& self)-> bool
   {
       return self.music.is_playing()
   }

   pub fn stop(&mut self )
   {
       self.music.stop();
   }

  pub fn give_position(&mut self,position: [f32; 3])
  {
    self.music.set_position(position)
  }

  pub fn up_volume(&mut self)
  {
     let mut now=self.get_vol();
     if now+0.1 <=  1 as f32
     {  now=now+0.1;
        self.set_vol(now);
     }

  }

  pub fn down_volume(&mut self)
  {
     let mut now=self.get_vol();
     if now-0.1 >= 0 as f32
     {  now=now-0.1;
        self.set_vol(now);
     }

  }

  pub fn get_vol(&self) -> f32
  {
     self.music.get_volume()
  }


  pub fn set_vol(&mut self, vol: f32)
  {
     self.music.set_volume(vol);
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

    pub fn new_from_data(datas: &SoundRessource)-> Self
    {
       Self{ data : datas.data.clone()}
    }

}

impl fmt::Debug for SoundRessource {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      
	write!(f, "SoundRessource")
    }
}



