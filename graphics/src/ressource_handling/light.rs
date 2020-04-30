use glium::uniforms::UniformBuffer;
pub const N_MAX_LIGHTS: usize = 128;

#[derive(Clone, Copy)]
pub enum Light
{
    Point(f32, [f32; 3], [f32; 3])
}

pub struct Lights
{
    pub light_type: UniformBuffer<[u32; N_MAX_LIGHTS]>,
    pub intensity: UniformBuffer<[f32; N_MAX_LIGHTS]>,
    pub position: UniformBuffer<[[f32; 3]; N_MAX_LIGHTS]>,
    pub direction: UniformBuffer<[[f32; 3]; N_MAX_LIGHTS]>,
    pub colour: UniformBuffer<[[f32; 3]; N_MAX_LIGHTS]>,
    pub n: u32,

}



impl Lights
{
    pub fn new(disp: &glium::Display) -> Self
    {
        Self
        {
            n: 0,
            light_type: UniformBuffer::new(disp, [0; N_MAX_LIGHTS]).unwrap(),
            intensity: UniformBuffer::new(disp, [0.; N_MAX_LIGHTS]).unwrap(),
            position: UniformBuffer::new(disp, [[0.; 3]; N_MAX_LIGHTS]).unwrap(),
            direction: UniformBuffer::new(disp, [[0.; 3]; N_MAX_LIGHTS]).unwrap(),
            colour: UniformBuffer::new(disp, [[0.; 3]; N_MAX_LIGHTS]).unwrap(),
        }
    }

    pub fn push(&mut self, light: Light)
    {
        if (self.n as usize) < N_MAX_LIGHTS-1
        {
            match light
            {
                Light::Point(intensity, pos, colour) =>
                {

                    let n = self.n as usize;

                    
                    self.light_type.map()[n] = 1;
                    self.intensity.map()[n] = intensity;
                    self.position.map()[n] = pos;
                    self.colour.map()[n] = colour;

                    self.n += 1;
                }
            }
        }
    }
    
    /// for debug purpose
    pub fn print(&mut self)
    {
        println!("LIGHTS:");
        println!("n = {}", self.n);
        print!("TYP: ");
        self.light_type.map().iter().for_each(|a| print!("{} ", a));
        print!("\nPOS: ");
        self.position.map().iter().for_each(|a| print!("{:?} ", a));
        print!("\nCOL: ");
        self.colour.map().iter().for_each(|a| print!("{:?} ", a));
//        println!("inte: {:?}", self.intensity.map()[n] = intensity);
  //      println!("posi: {:?}", self.position.map()[n] = pos);
    //    println!("colo: {:?}", self.colour.map()[n] = colour);
        
    }
    
}
