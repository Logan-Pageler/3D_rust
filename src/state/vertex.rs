// Define and store vertices
// Set up vertices and vertex buffers
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}


impl Vertex {
    // establish vertex description
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        // describe what the buffer looks like in memory
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,  // defined the width of the vertex
            step_mode: wgpu::VertexStepMode::Vertex,  // each element is per vertex
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,  // offset to attribute start
                    shader_location: 0,  // get the position field
                    format: wgpu::VertexFormat::Float32x3,  // size of attribute
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,  // get the color field
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}