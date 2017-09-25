#version 140

in vec2 position;
in vec3 color;

uniform vec2 offset;

out vec3 v_color;

void main() {
    gl_Position = vec4(position + offset, 0.0, 1.0);
/*    gl_Position = vec4(((vec3(position, 1.0) * matrix).xy - vec2(1, 1)) * vec2(0.25 * 0.75, 0.25), 0.0, 1.0);*/
    //gl_Position = vec4((vec3(position, 1.0) * matrix).xy, 0.0, 1.0);
    v_color = color;
}
