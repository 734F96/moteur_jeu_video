#version 140


smooth in vec3 v_normal;
smooth in vec3 v_position;

out vec4 f_color;

uniform vec3 ambiant;
uniform vec3 material_specular;
uniform float specular_exponent;

uniform vec3 emmission;
uniform vec3 diffuse;
uniform vec3 opacity;


uniform uint n_lights;

layout(std140) uniform lights_type
{
     uint l_type[128];
};
layout(std140) uniform lights_intensity
{
     float l_intensity[128];
};

layout(std140) uniform lights_pos
{
     vec4 l_pos[128];
};

layout(std140) uniform lights_dir
{
     vec4 l_dir[128]; 
};

layout(std140) uniform lights_col
{
     vec4 l_col[128];
};


/*
4 types of light:

* 1: Point (a position, no direction),

* 2: Ambiant (no position, no direction),

* 3: Spot (a position, a direction),

* 4: Parallel (no position, a direction),

All of them also have a colour and
an intensity.

A 0 in the type array == the end of the lights
*/
void main()
{
     vec3 material_diffuse = diffuse;

     vec3 norm = normalize(v_normal);
	



     vec3 color = vec3(0., 0., 0.);
         
     for (uint i = 0u; i < n_lights; ++i)
     {


	  vec3 light_color = l_col[i].xyz;
	  float light_intensity = l_col[i].w;
	  vec3 light_position = l_pos[i].xyz;
	  vec3 light_direction = l_dir[i].xyz;
	  float type = l_pos[i].w;
	  
	  
	  vec3 ambiant_col = light_color;
	  vec3 diffuse_col = material_diffuse * light_color;
	  vec3 specular_col = material_diffuse * light_color;

	  float ambiant_coef;
	  float diffuse_coef;
	  float specular_coef;
	  float intensity;

	  
	  if (type == 1.)
	  {

	       // the OPPOSITE of the ray's direction
	       vec3 light_dir = normalize(light_position - v_position);

	       // DIFFUSE
	       diffuse_coef = max(dot(norm, light_dir), 0.0);

	       // SPECULAR
	       vec3 camera_dir = normalize(-v_position);
	       vec3 half_direction = normalize(light_dir + camera_dir);


	       specular_coef =
		    pow(
			 max(dot(half_direction, norm), 0.0),
			 specular_exponent);
               // Ambiant
	       ambiant_coef = 0.0;
	       
	       
	       // GLOBAL INTENSITY
	       vec3 delta = light_position - v_position;
	       intensity = light_intensity
		    / dot(delta, delta);



	       color = min(
		    vec3(1., 1., 1.),
		    color +
		    intensity * (  diffuse_col * diffuse_coef
				   + specular_col* specular_coef
				   + ambiant_col * ambiant_coef ));



	  }
	  else if (type == 2.)
	  {
	       // DIFFUSE
	       diffuse_coef = 1.0;

	       // SPECULAR
	       specular_coef = 0.0;

	       // Ambiant
	       ambiant_coef = 1.0;

	       // GLOBAL INTENSITY
	       intensity = light_intensity;


	       color = min(
		    vec3(1., 1., 1.),
		    color + (ambiant_col*ambiant_coef
			     + diffuse_coef*diffuse_col
			 )*light_intensity
		    

		    );

	       
	  }
	  else if (type == 3.)
	  {

	       // the OPPOSITE of the ray's direction
	       vec3 light_dir = normalize(-light_direction);

	       // DIFFUSE
	       diffuse_coef = max(dot(norm, light_dir), 0.0);

	       // SPECULAR
	       vec3 camera_dir = normalize(-v_position);
	       vec3 half_direction = normalize(light_dir + camera_dir);

	       // Ambiant
	       ambiant_coef = 0.01;

	       specular_coef =
		    pow(
			 max(dot(half_direction, norm), 0.0),
			 specular_exponent);

	       // GLOBAL INTENSITY
	       vec3 delta = light_position - v_position;
	       intensity = light_intensity
		    / dot(delta, delta);




	       color = min(
		    vec3(1., 1., 1.),
		    color +
		    intensity * (  diffuse_col * diffuse_coef
				   + specular_col* specular_coef
				   + ambiant_col * ambiant_coef ));

	  }
	  else if (type == 4.)
	  {

	       // the OPPOSITE of the ray's direction
	       vec3 light_dir = normalize(-light_direction);

	       // DIFFUSE
	       diffuse_coef = max(dot(norm, light_dir), 0.0);

	       // SPECULAR
	       vec3 camera_dir = normalize(-v_position);
	       vec3 half_direction = normalize(light_dir + camera_dir);


	       specular_coef =
		    pow(
			 max(dot(half_direction, norm), 0.0),
			 specular_exponent);

	       // Ambiant
	       ambiant_coef = 0.01;
	       
	       // GLOBAL INTENSITY

	       intensity = light_intensity;




	       
	       color = min(
	       vec3(1., 1., 1.),
	       color +
	       intensity * (  diffuse_col * diffuse_coef
			    + specular_col* specular_coef
			    + ambiant_col * ambiant_coef ));

	  }
	    
     }
     
     f_color = vec4(color, 1.) ;
//   f_color = vec4(normalize(v_normal), 1.);
}
