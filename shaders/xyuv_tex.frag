#version 150

in vec2 f_texpos;
uniform sampler2D sampler;

out vec4 out_color;

void main() {
    out_color = texture(sampler, f_texpos);
}
