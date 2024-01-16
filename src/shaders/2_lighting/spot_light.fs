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
    vec3 position;
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float cutOff;
    float outerCutOff;

    float constant;
    float linear;
    float quadratic;
};

//in's
in vec3 fragPos;
in vec3 normal;
in vec2 texCoord;
//uniforms
uniform Material material;
uniform Light light;

//main
void main() {
    vec3 lightDir = normalize(light.position - fragPos);
    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);
    //ambient
    vec3 ambient = vec3(texture(material.diffuseTex, texCoord)) * light.ambient;
    //diffuse
    float diff = max(dot(lightDir, normalize(normal)), 0.0);
    vec3 diffuse = vec3(texture(material.diffuseTex, texCoord)) * diff * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normalize(normal));
    vec3 cameraDir = normalize(light.direction - fragPos);
    float spec = pow(max(dot(reflected, cameraDir), 0.0), material.shininess);
    vec3 specular = vec3(texture(material.specularTex, texCoord)) * spec * light.specular;
    //attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0/(light.constant + light.linear * distance + light.quadratic * distance * distance);
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    //smooth edge
    diffuse *= intensity;
    specular *= intensity;
    //result
    vec3 result = ambient + diffuse + specular;
    FragColor = vec4(result, 1.0);
}