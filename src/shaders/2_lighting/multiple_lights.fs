#version 330 core
//out
out vec4 FragColor;
//struct's
struct Material {
    sampler2D diffuseTex;
    sampler2D specularTex;
    float shininess;
};

struct DirectionalLight {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float constant;
    float linear;
    float quadratic;
};
#define NR_POINT_LIGHTS 4

struct SpotLight {
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
uniform DirectionalLight dirLight;
uniform PointLight pointLight[NR_POINT_LIGHTS];
uniform SpotLight spotLight;
uniform vec3 cameraPosition;

//function prototypes
vec3 CalcDirLight(DirectionalLight light, vec3 normal, vec3 viewDir);
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 fragPos);
vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 viewDir, vec3 fragPos);

//main
void main() {
    vec3 norm = normalize(normal);
    vec3 viewDir = normalize(cameraPosition - fragPos);

    vec3 direction = CalcDirLight(dirLight, norm, viewDir);
    vec3 points;
    for(int i = 0; i < NR_POINT_LIGHTS; i++) {
        points += CalcPointLight(pointLight[i], norm, viewDir, fragPos);
    }
    vec3 spot = CalcSpotLight(spotLight, norm, viewDir, fragPos);
    vec3 result = direction + points + spot;
    //result
    FragColor = vec4(result, 1.0);
}

vec3 CalcDirLight(DirectionalLight light, vec3 normal, vec3 viewDir) {
    vec3 lightDir = normalize(-light.direction);
    //ambient
    vec3 ambient = vec3(texture(material.diffuseTex, texCoord)) * light.ambient;
    //diffuse
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = vec3(texture(material.diffuseTex, texCoord)) * diff * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normal);
    float spec = pow(max(dot(reflected, viewDir), 0.0), material.shininess);
    vec3 specular = vec3(texture(material.specularTex, texCoord)) * spec * light.specular;
    //result
    return (ambient + diffuse + specular);
}

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 fragPos) {
    vec3 lightDir = normalize(light.position - fragPos);
    //ambient
    vec3 ambient = vec3(texture(material.diffuseTex, texCoord)) * light.ambient;
    //diffuse
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = vec3(texture(material.diffuseTex, texCoord)) * diff * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normal);
    float spec = pow(max(dot(reflected, viewDir), 0.0), material.shininess);
    vec3 specular = vec3(texture(material.specularTex, texCoord)) * spec * light.specular;
    //attentuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0/(light.constant + light.linear * distance + light.quadratic * distance * distance);
    //result
    return ((ambient + diffuse + specular) * attenuation);
}

vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 viewDir, vec3 fragPos) {
    vec3 lightDir = normalize(light.position - fragPos);
    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);
    //ambient
    vec3 ambient = vec3(texture(material.diffuseTex, texCoord)) * light.ambient;
    //diffuse
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = vec3(texture(material.diffuseTex, texCoord)) * diff * light.diffuse;
    //specular
    vec3 reflected = reflect(-lightDir, normal);
    float spec = pow(max(dot(reflected, viewDir), 0.0), material.shininess);
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
    return (ambient + diffuse + specular);
}
