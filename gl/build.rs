use std::{env, fs::File, path::Path};
use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator, DebugStructGenerator};


fn main() {
    let mut file = File::create(&Path::new(
        &env::var("OUT_DIR").unwrap()).join("bindings.rs")
    ).unwrap();

    let registry = Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [/*"GL_NV_command_list", */]);
 
    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry.write_bindings(DebugStructGenerator, &mut file).unwrap();
    } else {
        registry.write_bindings(StructGenerator, &mut file).unwrap();
    }
}
