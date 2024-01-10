#version 330
out vec4 FragColor;

in vec3 normal;
in vec3 fragPos;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 objectColor;

void main() 
{
    vec3 lightToFrag = normalize(lightPos - fragPos);
    float diffuseStrength = max(dot(normal, lightToFrag), 0.0);
    vec3 diffuse = diffuseStrength * lightColor;

    vec3 result = diffuse * objectColor;
    FragColor = vec4(result, 1.0);
}