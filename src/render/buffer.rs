
#[derive(Clone, Copy)]
pub enum DrawUsage {
    Static = 35044,
    Dynamic = 35048,
    Stream = 35040
}

pub type Array<T> = Buffer<ArrayType, T>;
pub type ElementArray<T> = Buffer<ElementArrayType, T>;

pub trait Type {
    const TYPE: gl::types::GLuint;
}

pub struct ArrayType;
impl Type for ArrayType {
    const TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct ElementArrayType;
impl Type for ElementArrayType {
    const TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}


pub struct Buffer<B: Type, T> {
    gl: gl::Gl,
    id: gl::types::GLuint,
    usage: DrawUsage,
    _marker_b: std::marker::PhantomData<B>,
    _marker_t: std::marker::PhantomData<T>
}

impl<B: Type, T> Buffer<B, T> {
    pub fn new(gl: &gl::Gl, usage: DrawUsage) -> Self {
        let mut id = 0;
        unsafe { gl.GenBuffers(1, &mut id) }
        Self {
            gl: gl.clone(),
            id,
            usage,
            _marker_b: std::marker::PhantomData,
            _marker_t: std::marker::PhantomData
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(B::TYPE, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(B::TYPE, 0) }
    }

    pub fn write_data(&self, data: &[T]) {
        self.bind();
        unsafe {
            self.gl.BufferData(
                B::TYPE,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage as u32
            );
        }
        self.unbind();
    }

    pub fn len(&self) -> i32 {
        let mut bytes = 0;
        self.bind();
        unsafe { self.gl.GetBufferParameteriv(B::TYPE, gl::BUFFER_SIZE, &mut bytes) };
        self.unbind();
        bytes / std::mem::size_of::<T>() as i32
    }
}

impl<B: Type, T> Drop for Buffer<B, T> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &mut self.id) }
    }
}
