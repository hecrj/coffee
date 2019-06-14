#version 150 core

in vec2 a_Pos;
in vec4 a_Color;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

out vec4 v_Color;

const mat4 INVERT_Y_AXIS = mat4(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, -1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(0.0, 0.0, 0.0, 1.0)
);

void main() {
    v_Color = a_Color;

    gl_Position = INVERT_Y_AXIS * u_MVP * vec4(a_Pos, 0.0, 1.0);
}
