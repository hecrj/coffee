#version 150 core

in vec4 v_Color;

out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

void main() {
    Target0 = v_Color;
}
