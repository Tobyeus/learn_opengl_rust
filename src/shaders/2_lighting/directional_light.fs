#version 330 core
//out
out vec4 FragColor;
//struct's
struct Material {
    sampler2D diffuseTex;
    sampler2D specularTex;
    float shininess;
};

struct Light {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

//in's
in vec3 fragPos;
in vec3 normal;
in vec2 texCoord;
//uniforms
uniform Material material;
uniform Light light;
uniform vec3 cameraPos;

//main
void main() {
    //ambient
    vec3 ambient = vec3(texture(material.diffuseTex, texCoord)) * light.ambient;
    //diffuse
    vec3 lightDir = normalize(-light.direction);
    float diff = max(dot(lightDir, normalize(normal)), 0.0);
    vec3 diffuse = vec3(texture(material.diffuseTex, texCoord)) * diff * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normalize(normal));
    vec3 cameraDir = normalize(cameraPos - fragPos);
    float spec = pow(max(dot(reflected, cameraDir), 0.0), material.shininess);
    vec3 specular = vec3(texture(material.specularTex, texCoord)) * spec * light.specular;
    //result
    FragColor = vec4((ambient + diffuse + specular), 1.0);
}