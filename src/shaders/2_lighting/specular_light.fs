#version 330
out vec4 FragColor;

in vec3 normal;
in vec3 fragPos;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 cameraPos;

void main() 
{
    float specularStrength = 0.5;
    vec3 lightDir = normalize(lightPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, normalize(normal));
    vec3 fragToCam = normalize(cameraPos - fragPos);
    float specular = pow(max(dot(reflectDir, fragToCam), 0.0), 32);

    vec3 result = specular * specularStrength * lightColor;
    FragColor = vec4(result, 1.0);
}