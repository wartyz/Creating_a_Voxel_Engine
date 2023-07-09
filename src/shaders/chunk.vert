#version 330 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in float voxel_id;// si no es float no va
layout (location = 2) in float face_id;

uniform mat4 m_proj;
uniform mat4 m_view;
uniform mat4 m_model;

out vec3 voxel_color;
out vec2 uv;

//const vec2 uv_coords[4] = vec2[4](
//vec2(0, 0), vec2(0, 1),
//vec2(1, 0), vec2(1, 1)
//);

const vec2 uv_coords[4] = vec2[4](
vec2(1, 0), vec2(1, 1),
vec2(0, 0), vec2(0, 1)
);

const int uv_indices[12] = int[12](
1, 0, 2, 1, 2, 3, // indices de coordenada de textura para vertices de caras pares
3, 0, 2, 3, 1, 0// caras impares
);




vec3 hash31(float p) {
    vec3 p3 = fract(vec3(p * 21.2) * vec3(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xxy + p3.yzz) * p3.zyx) + 0.05;
}




void main() {
    //int kk = face_id;

    //int uv_index = gl_VertexID % 6 + (face_id & 1) * 6;


    int uv_index = 0;
    if (face_id == 0){
        uv_index = gl_VertexID % 6;
    }
    if (face_id == 1){
        uv_index = gl_VertexID % 6 +  6;
    }
    if (face_id == 2){
        uv_index = gl_VertexID % 6;
    }
    if (face_id == 3){
        uv_index = gl_VertexID % 6 +  6;
    }
    if (face_id == 4){
        uv_index = gl_VertexID % 6;
    }
    if (face_id == 5){
        uv_index = gl_VertexID % 6 +  6;
    }

    // Solo funciona asi
    //int uv_index = gl_VertexID % 6 + 6;


    //    int uv_index = gl_VertexID % 6 + (kk) * 6;

    //float kk = bitwiseAND(vec2(48., 1.));

    uv = uv_coords[uv_indices[uv_index]];
    voxel_color = hash31(voxel_id);


    //voxel_color = vec3(0., 1., 0.);

    gl_Position = m_proj * m_view * m_model * vec4(in_position, 1.0);

}
