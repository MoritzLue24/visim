
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


pub use bindings::*;
use std::{rc::Rc, ops::Deref};


#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn load_with<F>(loadfn: F) -> Gl
where F: FnMut(&'static str) -> *const types::GLvoid {
    Gl {
        inner: Rc::new(bindings::Gl::load_with(loadfn))
    }
}
