#version 150 core

in vec2 a_Pos;

in vec4 a_Src;
in vec2 a_Scale;
in vec2 a_Translation;
in uint t_Layer;
in float a_Rotation;

layout(std140)uniform Globals{
    mat4 u_MVP;
};

out vec2 v_Uv;
flat out uint v_Layer;

void main(){
    v_Uv=a_Pos*a_Src.zw+a_Src.xy;
    v_Layer=t_Layer;
    
    mat4 Scale=mat4(
        vec4(a_Scale.x,0.,0.,0.),
        vec4(0.,a_Scale.y,0.,0.),
        vec4(0.,0.,1.,0.),
        vec4(0,0,0.,1.)
    );
    
    mat4 Rotate=mat4(
        vec4(cos(a_Rotation),-sin(a_Rotation),0.,0.),
        vec4(sin(a_Rotation),cos(a_Rotation),0.,0.),
        vec4(0.,0.,1.,0.),
        vec4(0.,0.,0.,1.)
    );
    
    mat4 Translate=mat4(
        vec4(1.,0.,0.,0.),
        vec4(0.,1.,0.,0.),
        vec4(0.,0.,1.,0.),
        vec4(a_Translation,0.,1.)
    );
    
    vec4 temp = (Rotate*vec4(a_Pos-vec2(.5,.5),0.,1.)) + vec4(.5,.5,0,0);
    mat4 instance_transform=Translate*Scale;
    
    vec4 position=u_MVP*instance_transform*temp;
    
    gl_Position=position;
}
