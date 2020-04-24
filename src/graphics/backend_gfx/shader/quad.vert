#version 150 core

in vec2 a_Pos;

in vec4 a_Src;
in vec2 a_Scale;
in vec2 a_Translation;
in uint t_Layer;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

out vec2 v_Uv;
flat out uint v_Layer;

void main() {
    v_Uv = a_Pos * a_Src.zw + a_Src.xy;
    v_Layer = t_Layer;

    mat4 instance_transform = mat4(
        vec4(a_Scale.x, 0.0, 0.0, 0.0),
        vec4(0.0, a_Scale.y, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(a_Translation, 0.0, 1.0)
    );

    vec4 position = u_MVP * instance_transform * vec4(a_Pos, 0.0, 1.0);

    gl_Position = position;
}
