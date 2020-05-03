#version 140

smooth in vec3 v_normal;
in vec2 v_tex_coords;
smooth in vec3 v_position;


out vec4 f_color;

uniform vec3 ambiant;
uniform vec3 specular;
uniform float specular_exponent;

uniform vec3 emmission;
uniform vec3 opacity;


uniform uint n_lights;

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

uniform sampler2D tex;

void main()
{
     vec3 diffuse = texture(tex, v_tex_coords).rgb;
     vec3 norm = normalize(v_normal);
     f_color = vec4(0);

     vec3 light_dir = vec3(0.);

     float light_distance = 0.;
    
    for (uint i = 0u; i < n_lights; ++i)
    {
	
	if (l_type[i] == 0u)
	{
	    break;
	}
	else
	{
	    light_dir = normalize(-l_pos[i]);
	    light_distance = distance(l_pos[i], v_position);

	}
	float diffuse_coef = max(dot(norm, light_dir), 0.0);

	vec3 camera_dir = normalize(-v_position);

	vec3 half_direction = normalize(light_dir + camera_dir);
	float specular_coef = pow(
	    max(dot(half_direction, norm),
		0.0),
	    specular_exponent);

	float attenuation = l_intensity[i]/pow(light_distance, 2);

	vec3 color = attenuation*(diffuse*diffuse_coef+specular*specular_coef);
	    
	f_color += vec4(color, 0.5) ;
   }
    
    
}
