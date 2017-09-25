#version 140

in vec2 position;

out vec2 texCoords;

uniform vec2 offset;

void main() {
    texCoords = position;
	gl_Position = vec4(position + offset, 0.0, 1.0);
}
