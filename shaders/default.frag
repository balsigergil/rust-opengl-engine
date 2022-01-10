#version 450 core

out vec4 outColor;

in vec3 color;
in vec2 texCoord;

uniform sampler2D uTexture;

void main()
{
    outColor = texture(uTexture, texCoord);
}