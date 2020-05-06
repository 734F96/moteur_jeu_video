use glium::uniforms::UniformBuffer;
pub const N_MAX_LIGHTS: usize = 128;



#[derive(Debug, Clone, Copy)]
pub enum Light
{
    NonDirectional(f32, [f32; 3]),
    Directional(f32, [f32; 3], [f32; 3]),
}

pub struct Lights
{
    pub light_type: UniformBuffer<[u32; N_MAX_LIGHTS]>,
    pub intensity: UniformBuffer<[f32; N_MAX_LIGHTS]>,
    pub position: UniformBuffer<[[f32; 4]; N_MAX_LIGHTS]>,
    pub direction: UniformBuffer<[[f32; 4]; N_MAX_LIGHTS]>,
    pub colour: UniformBuffer<[[f32; 4]; N_MAX_LIGHTS]>,
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
            position: UniformBuffer::new(disp, [[0.; 4]; N_MAX_LIGHTS]).unwrap(),
            direction: UniformBuffer::new(disp, [[0.; 4]; N_MAX_LIGHTS]).unwrap(),
            colour: UniformBuffer::new(disp, [[0.; 4]; N_MAX_LIGHTS]).unwrap(),
        }
    }

    pub fn push(&mut self, light: Light, maybe_pos: Option<([f32; 4], [f32; 4])>)
    {
        if (self.n as usize) < N_MAX_LIGHTS-1
        {
            match (light, maybe_pos)
            {
                (Light::NonDirectional(intensity, colour), Some((pos, _))) =>
                {

		    let colour = [colour[0], colour[1], colour[2], intensity];
                    let n = self.n as usize;

                    
                    self.intensity.map()[n] = intensity;
                    self.position.map()[n] = pos;
                    self.position.map()[n][3] = 1.;
                    self.colour.map()[n] = colour;

                    self.n += 1;
                },
                (Light::NonDirectional(intensity, colour), None) =>
                {

                    let n = self.n as usize;
		    let colour = [colour[0], colour[1], colour[2], intensity];

                    
                    self.position.map()[n][3] = 2.;
                    self.intensity.map()[n] = intensity;
                    self.colour.map()[n] = colour;

                    self.n += 1;
                },
                (Light::Directional(intensity, _, colour), Some((pos, dir))) =>
                {

		    let colour = [colour[0], colour[1], colour[2], intensity];
		    let dir = [dir[0], dir[1], dir[2], 0.];
                    let n = self.n as usize;

                    
                    self.intensity.map()[n] = intensity;
                    self.position.map()[n] = pos;
                    self.position.map()[n][3] = 3.;
                    self.direction.map()[n] = dir;
                    self.colour.map()[n] = colour;

                    self.n += 1;
                },
                (Light::Directional(intensity, dir, colour), None) =>
                {

		    let colour = [colour[0], colour[1], colour[2], intensity];
		    let dir = [dir[0], dir[1], dir[2], 0.];
                    let n = self.n as usize;

                    
                    self.intensity.map()[n] = intensity;
                    self.colour.map()[n] = colour;
                    self.direction.map()[n] = dir;
                    self.position.map()[n][3] = 4.;

                    self.n += 1;
                }
            }
        }
    }

    pub fn clear(&mut self)
    {
	self.n = 0;

        self.light_type.invalidate();
        self.intensity.invalidate();
        self.position.invalidate();
        self.colour.invalidate();
        self.direction.invalidate();
	
    }
    
    /// for debug purpose
    pub fn print(&mut self)
    {
        println!("LIGHTS:");
        println!("n = {}", self.n);
        print!("TYP: ");
        self.light_type.map().iter().for_each(|a| print!("{} ", a));
        print!("INT: ");
        self.intensity.map().iter().for_each(|a| print!("{} ", a));
        print!("\nPOS: ");
        self.position.map().iter().for_each(|a| print!("{:?} ", a));
        print!("\nCOL: ");
        self.colour.map().iter().for_each(|a| print!("{:?} ", a));
        print!("\nDIR: ");
        self.colour.map().iter().for_each(|a| print!("{:?} ", a));
//        println!("inte: {:?}", self.intensity.map()[n] = intensity);
  //      println!("posi: {:?}", self.position.map()[n] = pos);
    //    println!("colo: {:?}", self.colour.map()[n] = colour);
        
    }
    
}
