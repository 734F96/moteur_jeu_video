#version 140

in vec2 v_tex_coords;
out vec4 f_color;

uniform sampler2D tex;

void main() {
	f_color = texture(tex, v_tex_coords);
}
