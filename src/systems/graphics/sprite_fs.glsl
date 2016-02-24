#version 140

uniform vec4 tint;
uniform sampler2DArray tex;
uniform uint frame;

in vec2 v_tex_coords;

out vec4 f_color;

void main() {
    f_color = texture2DArray(tex, vec3(v_tex_coords, frame)) * tint;
}

