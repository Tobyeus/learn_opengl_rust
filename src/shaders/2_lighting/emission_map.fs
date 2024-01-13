#version 330 core
//out
out vec4 FragColor;
//struct's
struct Material {
    sampler2D diffuseTex;
    sampler2D specularTex;
    sampler2D emissionTex;
    float shininess;
};

struct Light {
    vec3 position;
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
    vec3 emission;
    //ambient
    vec3 ambient = vec3(texture(material.diffuseTex, texCoord)) * light.ambient;
    //diffuse
    vec3 lightDir = normalize(light.position - fragPos);
    float diff = max(dot(lightDir, normalize(normal)), 0.0);
    vec3 diffuse = vec3(texture(material.diffuseTex, texCoord)) * diff * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normalize(normal));
    vec3 cameraDir = normalize(cameraPos - fragPos);
    float spec = pow(max(dot(reflected, cameraDir), 0.0), material.shininess);
    vec3 specTex = vec3(texture(material.specularTex, texCoord));
    vec3 specular = specTex * spec * light.specular;
    if (specTex == vec3(0.0))
        emission = vec3(texture(material.emissionTex, texCoord));
    //result
    FragColor = vec4((ambient + diffuse + specular + emission), 1.0);
}