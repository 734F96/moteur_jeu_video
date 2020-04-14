#version 140

in vec3 position;
in vec3 normal;
in mat4 world_transformation;
in vec2 texture;

out vec2 v_tex_coords;
out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;

uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

uniform vec3 specular;
uniform float specular_exponent;
uniform float opacity;

const vec3 light_direction = normalize(vec3(0., 1., 1.));
const vec3 light_color = vec3(1., 1., 0.9);

void main() 
{
	vec3 norm = normalize((world_transformation*vec4(normal, 0.)).xyz);
	vec3 camera_dir = normalize((-world_transformation*vec4(position, 1.0)).xyz);
	vec3 half_direction = normalize(light_direction + camera_dir);
	float diffusion = max(dot(norm, light_direction), -dot(norm, light_direction)*(1.-opacity));

	float spec = pow(max(dot(half_direction, norm), 0.0), specular_exponent);

	v_tex_coords = texture;
	v_position = position;
	v_normal = norm;
	v_color = vec4(specular*spec, opacity);
	gl_Position = perspective_matrix*view_matrix*world_transformation*vec4(position, 1.0);
}
