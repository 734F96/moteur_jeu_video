#version 140

in vec3 v_normal;
in vec4 v_color;
in vec2 v_tex_coords;
out vec4 f_color;

const vec3 light_direction = normalize(vec3(0., 1., 1.));
const vec3 light_color = vec3(1., 1., 0.9);

uniform sampler2D tex;

void main() {
	float diffusion = max(dot(normalize(v_normal), light_direction), 0.01);
	f_color = texture(tex, v_tex_coords)*diffusion + v_color;
}
