#version 450

layout(location=0)in vec2 a_Pos;
layout(location=1)in vec4 a_Src;
layout(location=2)in vec2 a_Scale;
layout(location=3)in vec2 a_Translation;
layout(location=4)in float a_Rotation;
layout(location=5)in uint t_Layer;

layout(set=0,binding=0)uniform Globals{
    mat4 u_Transform;
};

layout(location=0)out vec2 v_Uv;
layout(location=1)flat out uint v_Layer;

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
    
    mat4 a_Transform=Translate*Rotate*Scale;
    
    gl_Position=u_Transform*a_Transform*vec4(a_Pos-vec2(.5,.5),0.,1.);
}
