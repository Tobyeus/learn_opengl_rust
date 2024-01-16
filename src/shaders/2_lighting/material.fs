#version 330 core
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;
    //intensities
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

out vec4 FragColor;

in vec3 normal;
in vec3 fragPos;

uniform Material material;
uniform Light light;
uniform vec3 cameraPos;

void main()
{   
    //ambient
    vec3 ambient = material.ambient * light.ambient;
    //diffuse
    vec3 lightDir = normalize(light.position - fragPos);
    float diff = max(dot(lightDir, normalize(normal)), 0.0);
    vec3 diffuse = (material.diffuse * diff) * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normalize(normal));
    vec3 cameraDir = normalize(cameraPos - fragPos);
    float spec = pow(max(dot(reflected, cameraDir), 0.0), material.shininess);
    vec3 specular = (material.specular * spec) * light.specular;
    //result
    FragColor = vec4((ambient + diffuse + specular), 1.0);
}