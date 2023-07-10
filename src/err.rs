use gl_dstruct::gl;


pub type Result<T> = std::result::Result<T, Error>;

macro_rules! gen_err_kind {
    (
        $(#[$enum_attr:meta])*
        pub enum Kind {
            $($var_ident:ident => fn $fn_ident:ident($($param_ident:ident: $param_ty:ty),*)),*
        }
    ) => {
        $(#[$enum_attr])*
        pub enum Kind {
            $($var_ident($($param_ty),*)),*
        }

        $(
            #[track_caller]
            pub fn $fn_ident<T: std::fmt::Debug>(msg: T$(, $param_ident: $param_ty)*) -> Error {
                let location = std::panic::Location::caller();
                Error { kind: Kind::$var_ident($($param_ident),*), msg: format!(
                    "{:?} at file: \"{}\", line: {}", msg, location.file(), location.line()
                ) }
            }
        )*
    };
}


pub extern "system" fn gl_debug_callback(
    _src: gl::types::GLenum,
    _ty: gl::types::GLenum,
    ident: gl::types::GLuint,
    severity: gl::types::GLenum,
    _len: gl::types::GLsizei,
    msg: *const gl::types::GLchar,
    _user_param: *mut std::ffi::c_void
) {
    if severity == gl::DEBUG_SEVERITY_NOTIFICATION {
        return
    }

    println!(
        "OpenGl error occurred\n{}",
        unsafe { std::ffi::CStr::from_ptr(msg) }.to_string_lossy()
    );
    std::process::exit(ident as i32);
}

gen_err_kind! {
    #[derive(Debug, PartialEq)]
    pub enum Kind {
        Other => fn new(),
        ParseShaderError => fn parse_shader(),
        LinkProgramError => fn link_program(),
        FileMissingError => fn file_missing(path: String),
        TypeConvertError => fn type_convert()
    }
}

pub struct Error {
    pub kind: Kind,
    pub msg: String 
}

impl std::error::Error for Error { }

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.msg)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.msg)
    }
}
