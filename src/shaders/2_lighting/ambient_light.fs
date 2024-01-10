#version 330
out vec4 FragColor;

uniform vec3 lightColor;
uniform vec3 objectColor;

void main() 
{
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    vec3 result = ambient * objectColor;
    FragColor = vec4(result, 1.0);
}