#version 330 core

in vec4 v_Color;
flat in int v_TexIndex;
in vec2 v_TexCoords;

uniform sampler2D u_Textures[1];
uniform sampler2D u_Texture;

out vec4 o_Color;

void main() {
	if (v_TexIndex == -1) {
		o_Color = v_Color;
	}
	else {
		o_Color = texture(u_Texture, v_TexCoords);
	}
}
