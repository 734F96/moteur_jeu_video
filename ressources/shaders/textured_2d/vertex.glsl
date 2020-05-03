#version 140

in vec3 position;
in vec2 texture;
in mat4 world_transformation;
out vec2 v_tex_coords;
out vec3 v_position;

uniform mat4 view_matrix;
uniform mat4 perspective_matrix;


void main()
{
     /*
	v_tex_coords = texture;
	v_position = position;
//	gl_Position = vec4(position, 1.0);
	
     gl_Position =
	  perspective_matrix *
	  world_transformation *
	  vec4(position, 1.);
*/

          // for non-uniform scaling
     
     vec4 world_position = world_transformation * vec4(position, 1.0);
//     v_position = world_position.xyz / world_position.w;
     v_position = position;	
     gl_Position =
	  perspective_matrix
//	  *view_matrix
	  *world_position;

     v_tex_coords = texture;

}
