#version 140

in vec2 texCoords;

uniform sampler2D image;

out vec4 color;

void main() {
	color = texelFetch(image,  ivec2(vec2(texCoords[0], texCoords[1]) * textureSize(image, 0)), 0);
}
