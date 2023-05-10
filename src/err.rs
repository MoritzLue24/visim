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
            pub fn $fn_ident<T: std::fmt::Debug>(msg: T$(, $param_ident: $param_ty)*) -> Error {
                let location = std::panic::Location::caller();
                Error { kind: Kind::$var_ident($($param_ident),*), msg: format!(
                    "{:?} at file: \"{}\", line: {}", msg, location.file(), location.line()
                ) }
            }
        )*
    };
}

#[derive(Debug, PartialEq)]
pub enum GlCode {
    InvalidEnum = 1280,
    InvalidValue = 1281,
    InvalidOperation = 1282,
    StackOverflow = 1283,
    StackUnderflow = 1284,
    OutOfMemory = 1285,
    InvalidFramebufferOperation = 1286,
    ContextLost = 1287,
    TableTooLarge = 32817,
}

gen_err_kind! {
    #[derive(Debug, PartialEq)]
    pub enum Kind {
        Other => fn new(),
        ParseShaderError => fn parse_shader(),
        LinkProgramError => fn link_program(),
        OpenGl => fn open_gl(code: GlCode)
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
