#version 140

in vec3 position;
in vec3 normal;
in mat4 world_transformation;

out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;

uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

uniform vec3 ambiant; // remarque, ce n'est pas le rôle de l'objet de savoir l'ambiance
uniform vec3 diffuse;
uniform vec3 specular;
uniform float specular_exponent;
uniform vec3 emission;
uniform float opacity;
const vec3 light_direction = normalize(vec3(0., 1., -1.));

const vec3 light_color = vec3(1., 1., 0.9);

void main()
{
	vec3 norm = normalize((world_transformation*vec4(normal, 0.)).xyz);

	float diffusion = max(dot(norm, light_direction), 0.);
	vec3 camera_dir = normalize((-view_matrix[3]-world_transformation*vec4(position, 1.0)).xyz);
	vec3 half_direction = normalize(normalize(light_direction) + camera_dir);

	float spec = pow(max(dot(half_direction, norm), 0.0), specular_exponent);
	v_position = position;
	v_normal = norm;
	v_color = vec4(diffuse*0.01 + diffuse*diffusion + specular*spec, opacity);
	gl_Position = perspective_matrix*view_matrix*world_transformation*vec4(position, 1.0);
}
