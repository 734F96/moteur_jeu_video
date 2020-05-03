#version 140

in vec3 position;
in vec3 normal;
in mat4 world_transformation;
in vec2 texture;

out vec2 v_tex_coords;
smooth out vec3 v_position;
smooth out vec3 v_normal;

uniform mat4 view_matrix;
uniform mat4 perspective_matrix;


void main()
{
     // for non-uniform scaling
     v_normal = normalize(transpose(inverse(mat3(world_transformation))) * normal);
     
     vec4 world_position = world_transformation * vec4(position, 1.0);
     v_position = world_position.xyz / world_position.w;
	
     gl_Position =
	  perspective_matrix
	  *view_matrix
	  *world_position;

     v_tex_coords = texture;
}
