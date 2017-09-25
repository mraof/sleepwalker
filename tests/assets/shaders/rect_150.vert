#version 130 

in vec2 a_Pos;
in vec2 a_Uv;
out vec2 v_Uv;
uniform float f_Time;

void main() {
    v_Uv = a_Uv;
    float time = sin(f_Time / 12.0) * 32.0;
    gl_Position = vec4((a_Pos.x - 0.1) * f_Time * f_Time / 20 - 0.8, a_Pos.y * f_Time * f_Time / 20 + abs(sin(time)) - 1.0, 0.0, 1.0);
}
