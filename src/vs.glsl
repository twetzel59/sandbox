layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

out vec4 pass_color;

uniform float time;
uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 projection_mat;

void main(void) {
    gl_Position = projection_mat * view_mat * model_mat * vec4(pos, 1.0);
    
    pass_color = vec4(color * max(0.2, sin(time)), 1.0);
}
