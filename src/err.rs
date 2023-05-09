pub type Result<T> = std::result::Result<T, Error>;


macro_rules! gen_err_fn {
    ($(pub fn $fn_ident:ident => Kind::$var_ident:ident),*) => {
        $(
            #[track_caller]
            pub fn $fn_ident<T: std::fmt::Debug>(msg: T) -> Error {
                let location = std::panic::Location::caller();
                Error { kind: Kind::$var_ident, msg: format!(
                    "\"{:?}\" at file: \"{}\", line: {}", msg, location.file(), location.line()
                )}
            }
        )*
    };
}

gen_err_fn! {
    pub fn new => Kind::Other,
    pub fn parse_shader => Kind::ParseShaderError,
    pub fn link_program => Kind::LinkProgramError
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Other,
    ParseShaderError,
    LinkProgramError,
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
