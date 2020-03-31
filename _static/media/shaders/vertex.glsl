#version 300 es
precision mediump float;

/*
    Each draw call will run this vertex shader 4 times
    e.g. once per each vertex of the unit quad

    In order to draw the appropriate cell
    The uv coords of _that_ quad (in the bitmap)
    Need to be passed in as well
*/

in vec2 a_geom_vertex;
in vec2 a_tex_vertex;

out vec2 v_uv;

uniform vec2 u_position;
uniform mat4 u_size;
uniform mat4 u_camera;

void main() {

    mat4 transform = mat4(1.0);

    //https://www.geeks3d.com/20141114/glsl-4x4-matrix-mat4-fields/
    transform[3] = vec4(u_position, 0.0, 1.0);

    mat4 modelViewProjection = u_camera * transform; 

    gl_Position = modelViewProjection * (u_size * vec4(a_geom_vertex,0,1));
    v_uv = a_tex_vertex;
    //v_uv = a_geom_vertex;
}