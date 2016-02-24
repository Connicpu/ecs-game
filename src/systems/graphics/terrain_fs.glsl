#version 140

uniform sampler2DArray tex;

in vec3 v_tex_coords;

out vec4 f_color;

void main() {
    f_color = texture2DArray(tex, v_tex_coords);
}

