#version 450 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec3 inColor;
layout (location = 3) in vec2 inTexCoord;

uniform mat4 uMVP;

void main()
{
    gl_Position = uMVP * vec4(inPosition, 1.0f);
}