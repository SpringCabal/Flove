uniform vec3 treeDeformation; // sway, grown percentage, model height

void main(void)
{
    vec4 vertex = deformTree(gl_Vertex, treeDeformation.x, treeDeformation.y, treeDeformation.z);
    gl_Position = gl_ModelViewProjectionMatrix * vertex;
    gl_FrontColor = gl_Color;
    gl_BackColor = gl_Color;
}
