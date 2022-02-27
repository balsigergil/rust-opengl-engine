#version 450 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec3 inColor;
layout (location = 3) in vec2 inTexCoord;

out vec3 color;
out vec2 texCoord;
out vec3 normal;
out vec3 position;

uniform mat4 uCameraViewProjection;
uniform mat4 uModel;

void main()
{
    position = vec3(uModel * vec4(inPosition, 1.0f));
    color = inColor;
    texCoord = inTexCoord;
    normal = inNormal;
    gl_Position = uCameraViewProjection * uModel * vec4(inPosition, 1.0f);
}