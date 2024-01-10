#version 330
out vec4 FragColor;

in vec3 normal;
in vec3 fragPos;

uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 lightPos;
uniform vec3 cameraPos;

void main()
{   
    // ambient light
    // paramter multipy with lightColor
    float ambientParameter = 0.1;
    vec3 ambient = ambientParameter * lightColor;
    // diffuse light
    // dot product between lightSourceDir and Normals
    vec3 lightDir = normalize(lightPos - fragPos);
    float diffuseParameter = max(dot(lightDir, normalize(normal)), 0.0);
    vec3 diffuse = diffuseParameter * lightColor;
    // specular light
    // dot product between reflected(on object) light vector and view direction(camera)
    float specularParameter = 0.5;
    vec3 reflected = reflect(-lightDir, normalize(normal));
    vec3 cameraDir = normalize(cameraPos - fragPos);
    float specular = pow(max(dot(reflected, cameraDir), 0.0), 32);
    vec3 spec = specularParameter * specular * lightColor;
    // combine ambient, diffuse and specular -> Phong lighting
    FragColor = vec4((ambient + diffuse + spec) * objectColor, 1.0); 
}