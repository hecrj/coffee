#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;

in vec4 a_Src;
in vec3 a_TCol1;
in vec3 a_TCol2;
in vec3 a_TCol3;
in vec4 a_Color;
in uint t_Layer;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

out vec2 v_Uv;
out vec4 v_Color;
flat out uint v_Layer;

const mat4 invert_y_axis = mat4(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, -1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(0.0, 0.0, 0.0, 1.0)
);

void main() {
    v_Uv = a_Uv * a_Src.zw + a_Src.xy;
    v_Color = a_Color;
    v_Layer = t_Layer;

    mat4 instance_transform = mat4(
        vec4(a_TCol1, 0.0),
        vec4(a_TCol2, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(a_TCol3, 1.0)
    );

    vec4 position = invert_y_axis * u_MVP * instance_transform * vec4(a_Pos, 0.0, 1.0);

    gl_Position = position;
}
