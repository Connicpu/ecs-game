#version 140

uniform mat4 matrix;

// Vertex data
in vec2 position;
in vec2 tex_coords;
// Instance data
in vec2 offset;
in uint tile;

out vec3 v_tex_coords;

void main() {
    gl_Position = vec4(position + offset, 0.0, 1.0) * matrix;
    v_tex_coords = vec3(tex_coords, tile);
}

