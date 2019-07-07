in vec2 pass_uv;

out vec4 frag_color;

uniform sampler2D terrain_texture;

void main(void) {
    frag_color = vec4(texture(terrain_texture, pass_uv).rgb, 1.0);
    
    /*
    frag_color += vec4(2.0, 2.0, 2.0, 2.0);
    
    frag_color.r = min(frag_color.r, 1.0);
    frag_color.g = min(frag_color.g, 1.0);
    frag_color.b = min(frag_color.b, 1.0);
    frag_color.a = min(frag_color.a, 1.0);
    
    //if(frag_color.r == 1.0) {
    //    frag_color = vec4(0.0, 1.0, 0.0, 1.0);
    //}
    
    frag_color *= vec4(pass_uv, 0.0, 1.0);
    * */
}
