#ifndef MESH_HLSL
#define MESH_HLSL

struct Mesh{
  uint pos_offset;
  uint uv_offset;
  uint normal_offset;
  uint colors_offset;
  uint tangents_offset;
  uint materials_data_offset;
  uint material_ids_offset;
};

// mesh material 
#define MAPS_COUNT 5
#define NORMAL_MAP_INDEX 0
#define SPECULAR_MAP_INDEX 1
#define ALBEDO_MAP_INDEX 2
#define EMISSIVE_MAP_INDEX 3
#define OCCLUSION_MAP_INDEX 4

struct MeshMaterial {
  float4 base_color_factor;
  uint maps_index[MAPS_COUNT];
  float roughness_factor;
  float metalness_factor;
  float3 emissive_factors;
};

#endif