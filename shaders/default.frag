#version 450 core

out vec4 outColor;

in vec3 color;
in vec2 texCoord;
in vec3 normal;
in vec3 position;

uniform sampler2D uTexture;
uniform vec3 uLightPosition;
uniform vec3 uLightColor;

void main()
{
    // Ambient light
    float ambientStrength = 0.2f;
    vec3 ambient = ambientStrength * uLightColor;

    // Diffuse light
    vec3 lightDirection = normalize(uLightPosition - position);
    float diffuseAmount = max(dot(normalize(normal), lightDirection), 0.0);
    vec3 diffuse = diffuseAmount * uLightColor;

    outColor = vec4(ambient + diffuse, 1.0) * texture(uTexture, texCoord);
}