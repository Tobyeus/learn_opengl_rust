#version 330 core
out vec4 FragColor;

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

in vec2 TexCoords;
in vec3 Normals;
in vec3 FragPos;

uniform sampler2D texture_diffuse1;
uniform sampler2D texture_specular1;
uniform Light light;
uniform vec3 cameraPos;

void main()
{   
    vec3 normal = normalize(Normals);
    vec3 fragToLight = normalize(light.position - FragPos);
    vec3 cameraDir = normalize(cameraPos - FragPos);
    //ambient
    vec3 ambient = vec3(texture(texture_diffuse1, TexCoords)) * light.ambient;
    //diffuse
    float diff_factor = max(dot(fragToLight, normal), 0.0);
    vec3 diffuse = vec3(texture(texture_diffuse1, TexCoords)) * diff_factor * light.diffuse;
    //specular
    vec3 reflected = reflect(-fragToLight, normal);
    float spec_factor = pow(max(dot(reflected, cameraDir), 0.0), 16);
    vec3 specular = vec3(texture(texture_specular1, TexCoords)) * spec_factor * light.specular;
    //attenuation
    float distance = length(light.position - FragPos);
    float attenuation = 1 / (light.constant + (light.linear * distance) + (light.quadratic * distance));
    //vec4 result = texture(texture_diffuse1, TexCoords);
    vec3 result = (ambient + diffuse + specular) * attenuation;
    FragColor = vec4(result, 1.0);
}