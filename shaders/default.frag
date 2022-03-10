#version 450 core

out vec4 outColor;

in vec3 color;
in vec2 texCoord;
in vec3 normal;
in vec3 position;

uniform sampler2D uTextureDiffuse;
uniform sampler2D uTextureSpecular;
uniform vec3 uLightPosition;
uniform vec3 uLightColor;
uniform vec3 uCameraPosition;

// Parameters
const float ambientStrength = 0.2f;
const float shininess = 16.0;
const vec3 specColor = vec3(1.0f, 1.0f, 1.0f);
const float specDistanceFactor = 1.0f;
const float specMapAdjustment = 1.5f;

void main()
{
    vec3 normalizedNormal = normalize(normal);

    // Ambient light
    vec3 ambient = ambientStrength * uLightColor;

    // Light direction and distance
    vec3 lightDirection = uLightPosition - position;
    float distance = length(lightDirection);
    distance = distance * distance;
    lightDirection = normalize(lightDirection);

    // Diffuse light
    float diffuseAmount = max(dot(normalizedNormal, lightDirection), 0.0);
    vec3 diffuse = diffuseAmount * uLightColor;

    // Specular lighting
    float lambertian = max(dot(lightDirection, normalizedNormal), 0.0);
    float specularAmount = 0.0f;
    vec3 specular = vec3(0.0f, 0.0f, 0.0f);

    if (lambertian > 0.0) {
        vec3 viewDirection = normalize(uCameraPosition - position);

        // Blinn-Phong
        vec3 halfDirection = normalize(lightDirection + viewDirection);
        float specAngle = max(dot(halfDirection, normalizedNormal), 0.0);
        specularAmount = pow(specAngle, shininess);
        specular = specularAmount * specColor * (vec3(texture(uTextureSpecular, texCoord)) + specMapAdjustment);
    }

    outColor = vec4(ambient + diffuse + specular / max(distance * specDistanceFactor, 1.0f), 1.0) * texture(uTextureDiffuse, texCoord);
}