#version 140

in vec3 position;
in vec3 normal;
in mat4 world_transformation;
out vec3 v_position;

uniform mat4 view_matrix;


void main() {
v_position = position;
gl_Position = view_matrix*world_transformation*vec4(position, 1.0);
}
