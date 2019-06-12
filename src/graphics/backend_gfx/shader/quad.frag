#version 150 core

uniform sampler2DArray t_Texture;
flat in uint v_Layer;
in vec2 v_Uv;

out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

void main() {
    Target0 = texture(t_Texture, vec3(v_Uv, v_Layer));
}
