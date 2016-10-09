#version 150

in vec2 f_texpos;
uniform sampler2D sampler;

out vec4 out_color;

void main() {
    float intensity = texture(sampler, f_texpos).x;
    out_color = vec4(vec3(intensity), 1);
}
