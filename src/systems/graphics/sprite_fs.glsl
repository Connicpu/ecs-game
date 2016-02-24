#version 140

uniform vec4 tint;
uniform sampler2D tex;
uniform float frame;

in vec2 v_tex_coords;

out vec4 f_color;

void main() {
    f_color = texture(tex, v_tex_coords) * tint;
}

