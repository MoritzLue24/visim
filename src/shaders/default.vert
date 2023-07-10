#version 330 core

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec4 a_Color;
layout (location = 2) in int a_TexIndex;
layout (location = 3) in vec2 a_TexCoords;

out vec4 v_Color;
flat out int v_TexIndex;
out vec2 v_TexCoords;

void main() {
	v_Color = a_Color;
	v_TexIndex = a_TexIndex;
	v_TexCoords = a_TexCoords;
    gl_Position = vec4(a_Position, 1.0, 1.0);
}
