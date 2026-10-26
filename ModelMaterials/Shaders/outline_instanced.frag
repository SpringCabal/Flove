#version 430 core
uniform sampler2D tex0;
uniform vec2 screenXY;
in vec4 outlineColor;
out vec4 fragColor;
void main() {
    if (texture(tex0, gl_FragCoord.xy / screenXY).z < gl_FragCoord.z) discard;
    fragColor = outlineColor;
}
