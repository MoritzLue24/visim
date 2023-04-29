
#[derive(Clone, Copy)]
pub enum DrawUsage {
    Static = 35044,
    Dynamic = 35048,
    Stream = 35040
}

pub type Array = Buffer<ArrayType>;
pub type ElementArray = Buffer<ElementArrayType>;

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


pub struct Buffer<T: Type> {
    gl: gl::Gl,
    id: gl::types::GLuint,
    usage: DrawUsage,
    _marker: std::marker::PhantomData<T>
}

impl<T: Type> Buffer<T> {
    pub fn new(gl: &gl::Gl, usage: DrawUsage) -> Self {
        let mut id = 0;
        unsafe { gl.GenBuffers(1, &mut id) }
        Self { gl: gl.clone(), id, usage, _marker: std::marker::PhantomData }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(T::TYPE, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(T::TYPE, 0) }
    }

    pub fn write_data<U>(&self, data: &[U]) {
        unsafe {
            self.gl.BufferData(
                T::TYPE,
                (data.len() * std::mem::size_of::<U>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage as u32
            );
        }
    }
}

impl<T: Type> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &mut self.id) }
    }
}
