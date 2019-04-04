#version 150 core

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

const vec2 QUAD_VERTS[4] = vec2[](
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(1.0, 1.0),
    vec2(0.0, 1.0)
);

const mat4 INVERT_Y_AXIS = mat4(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, -1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(0.0, 0.0, 0.0, 1.0)
);

void main() {
    vec2 v_Pos = QUAD_VERTS[gl_VertexID % 4];
    v_Uv = v_Pos * a_Src.zw + a_Src.xy;
    v_Color = a_Color;
    v_Layer = t_Layer;

    mat4 instance_transform = mat4(
        vec4(a_TCol1, 0.0),
        vec4(a_TCol2, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(a_TCol3, 1.0)
    );

    vec4 position = INVERT_Y_AXIS * u_MVP * instance_transform * vec4(v_Pos, 0.0, 1.0);

    gl_Position = position;
}
