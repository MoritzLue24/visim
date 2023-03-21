
pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct ArrayBufferType;
impl BufferType for ArrayBufferType {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct ElementArrayBufferType;
impl BufferType for ElementArrayBufferType {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}


pub struct Buffer<T: BufferType> {
    gl: gl::Gl,
    id: gl::types::GLuint,
    _marker: std::marker::PhantomData<T>
}

impl<T: BufferType> Buffer<T> {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut id = 0;
        unsafe { gl.GenBuffers(1, &mut id) }
        Self { gl: gl.clone(), id, _marker: std::marker::PhantomData }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(T::BUFFER_TYPE, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(T::BUFFER_TYPE, 0) }
    }

    pub fn static_draw_data<U>(&self, data: &[U]) {
        unsafe {
            self.gl.BufferData(
                T::BUFFER_TYPE,
                (data.len() * std::mem::size_of::<U>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
    }
}

impl<T: BufferType> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &mut self.id) }
    }
}
