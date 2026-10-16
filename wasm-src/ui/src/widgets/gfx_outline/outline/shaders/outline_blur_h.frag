uniform sampler2D tex0;
uniform int screenX;

const vec2 kernel = vec2(0.6, 0.7);

// Filter premultiplied colours so a coloured hover rim retains its hue.
vec4 sampleMask(vec2 uv) {
    vec4 c = texture2D(tex0, uv);
    return vec4(c.rgb * c.a, c.a);
}

void main(void) {
    vec2 texCoord = vec2(gl_TextureMatrix[0] * gl_TexCoord[0]);
    gl_FragColor = vec4(0.0);
    float pixelsize = 1.0 / float(screenX);
    gl_FragColor += kernel[0] * sampleMask(vec2(texCoord.s + 2.0 * pixelsize, texCoord.t));
    gl_FragColor += kernel[1] * sampleMask(vec2(texCoord.s + pixelsize, texCoord.t));
    gl_FragColor += sampleMask(texCoord);
    gl_FragColor += kernel[1] * sampleMask(vec2(texCoord.s - 1.0 * pixelsize, texCoord.t));
    gl_FragColor += kernel[0] * sampleMask(vec2(texCoord.s - 2.0 * pixelsize, texCoord.t));
    gl_FragColor.rgb /= max(gl_FragColor.a, 0.00001);
}
