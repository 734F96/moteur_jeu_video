#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 f_color;

uniform vec3 ambiant;
uniform vec3 diffuse;
uniform vec3 specular;
uniform float specular_exponent;

uniform vec3 emmission;
uniform vec3 opacity;


layout(std140) uniform lights_types
{
    uint l_type[128];
	
};
layout(std140) uniform lights_intensity
{
    float l_intensity[128];
};
layout(std140) uniform lights_dir
{
    vec3 l_dir[128]; 
};
layout(std140) uniform lights_pos
{
    vec3 l_pos[128];
};

layout(std140) uniform lights_col
{
    vec3 l_col[128];
};

void main() {
    vec3 norm = normalize(v_normal);
    f_color = vec4(0);

    vec3 light_dir = vec3(0.);
    
    for (int i = 0; i < 100; ++i)
    {
	
	if (l_type[i] == 0u)
	{
	    break;
	}
	else
	{
	    light_dir = normalize(-l_pos[i]);
	}
	float diffuse_coef = max(dot(norm, light_dir), 0.0);

	vec3 camera_dir = normalize(-v_position);

	vec3 half_direction = normalize(light_dir + camera_dir);
	float specular = pow(
	    max(dot(half_direction, norm),
		0.0),
	    16.0);

/*
	  vec3 half_direction = normalize(normalize(u_light) + camera_dir);
	  float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
	*/

	vec3 color = diffuse*diffuse_coef+specular*vec3(1.);

	    
	f_color += vec4(color, 1.) ;
   }
    
    
}
