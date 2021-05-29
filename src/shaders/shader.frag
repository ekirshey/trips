// shader.frag
#version 450

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) 
uniform Uniforms {
    vec4 in_color;
};

void main() {
    f_color = in_color;
}
