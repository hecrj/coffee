#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 1) flat in uint v_Layer;

layout(set = 0, binding = 1) uniform sampler u_Sampler;
layout(set = 1, binding = 0) uniform texture2DArray u_Texture;

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = texture(sampler2DArray(u_Texture, u_Sampler), vec3(v_Uv, v_Layer));
}
