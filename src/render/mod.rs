mod vertex;
mod shader;
mod program;
mod buffer;
mod vertex_array;

pub use vertex::Vertex;
pub use shader::Shader;
pub use program::Program;
pub use buffer::{Buffer, ArrayBufferType, ElementArrayBufferType};
pub use vertex_array::VertexArray;

pub type ArrayBuffer = Buffer<ArrayBufferType>;
pub type ElementArrayBuffer = Buffer<ElementArrayBufferType>;