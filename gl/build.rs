use std::{env, fs::File, path::Path};
use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator};


fn main() {
    let mut file = File::create(&Path::new(
        &env::var("OUT_DIR").unwrap()).join("bindings.rs")
    ).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [/*"GL_NV_command_list", */])
        .write_bindings(StructGenerator, &mut file).unwrap();
}
