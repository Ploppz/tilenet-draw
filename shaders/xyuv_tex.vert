#version 150

in vec2 pos;
in vec2 texpos;

out vec2 f_texpos; // fragment texture position


void main() {
    f_texpos = texpos;
    gl_Position = vec4(pos, 0, 1);
}
