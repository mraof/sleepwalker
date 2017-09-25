#version 140

in vec2 texCoords;

uniform sampler1D palette;
uniform usampler1D image;
uniform int width;
uniform int height;
uniform uint size;


out vec4 color;

void main() {
    ivec2 coords = ivec2(texCoords * ivec2(width, height));
    int offset = (coords.y * width + coords.x) * int(size);
    int byteoffset = int(offset / 8);
    uint bitoffset = uint(offset % 8);
    color = texelFetch(palette, int(((texelFetch(image, byteoffset, 0)
     << bitoffset) & 0xffu).r >> (8u - size) |
        texelFetch(image, byteoffset + 1, 0).r >> (16u - bitoffset - size)), 0);
}
