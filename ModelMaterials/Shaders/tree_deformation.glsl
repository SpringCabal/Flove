// Shared by the material and silhouette passes: deformation is model-local.
vec4 deformTree(vec4 vertex, float sway, float grown, float height)
{
    float distanceFromTrunk = (abs(vertex.x) + abs(vertex.z)) / 10.0;
    vertex.x += distanceFromTrunk * vertex.y * sway;
    vertex.z += distanceFromTrunk * vertex.y * sway;
    vertex.x += sin(distanceFromTrunk) * (1.0 - grown) * 10.0;
    vertex.x += cos(distanceFromTrunk) * (1.0 - grown) * 10.0;
    vertex.y -= (1.0 - grown) * height;
    return vertex;
}
