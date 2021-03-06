use glam::Vec3;

#[allow(dead_code)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct Camera {
    pub view: [f32; 16],
    pub proj: [f32; 16],
    pub cam_pos: [f32; 3],
    pub light_positions: [[f32; 3]; 4],
    pub light_colors: [[f32; 3]; 4],
}
#[allow(dead_code)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct GpuMesh {
    pub pos_offset: u32,
    pub uv_offset: u32,
    pub normal_offset: u32,
    pub colors_offset: u32,
    pub tangents_offset: u32,
    pub materials_data_offset: u32,
    pub material_ids_offset: u32,
}

impl crate::buffer::BufferData for GpuMesh {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<GpuMesh>(),
            )
        }
    }

    fn bsize(&self) -> usize {
        std::mem::size_of::<GpuMesh>()
    }
}
