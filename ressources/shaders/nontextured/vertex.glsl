#version 140

in vec3 position;
in vec3 normal;
in mat4 world_transformation;

out vec3 v_position;
out vec3 v_normal;

uniform mat4 view_matrix;
uniform mat4 perspective_matrix;


void main()
{
	vec3 norm = normalize((world_transformation*vec4(normal, 0.)).xyz);

	//v_normal = norm;
	v_normal = normalize(transpose(inverse(mat3(world_transformation))) * normal);
	gl_Position =
	    perspective_matrix
	    *view_matrix
	    *world_transformation
	    *vec4(position, 1.0);

	v_position = gl_Position.xyz / gl_Position.w;
}
