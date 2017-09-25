#version 130

in vec2 v_Uv;
out vec4 Target0;

uniform sampler2D t_Texture;
uniform float f_Time;

void main() {
    float time = sin(f_Time / 12.0) * 32.0;
    Target0 = texture(t_Texture, v_Uv);
}
