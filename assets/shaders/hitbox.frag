#version 140

uniform vec3 u_color;

out vec4 color;

void main() {
    color = vec4(u_color, 0.7);
}
