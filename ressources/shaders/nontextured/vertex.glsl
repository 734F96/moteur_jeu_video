#version 140

in vec3 position;
in vec3 normal;
in mat4 world_transformation;

smooth out vec3 v_position;
smooth out vec3 v_normal;

uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

void main()
{
     v_normal = transpose(inverse(mat3(world_transformation))) * normal;
     v_position = vec3((world_transformation * vec4(position, 1.)));

     gl_Position =
	  perspective_matrix
	  * view_matrix
	  * world_transformation
	  * vec4(position, 1.);

}
