#version 330
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 normal;
out vec3 fragPos;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() 
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    //transforming normals with the model
    normal = mat3(transpose(inverse(model))) * aNormal;  
    fragPos =  vec3(model * vec4(aPos, 1.0));
}