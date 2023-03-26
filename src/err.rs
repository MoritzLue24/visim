
#[track_caller]
pub fn new<T: std::fmt::Debug>(msg: T) -> Error {
    let location = std::panic::Location::caller();
    Error { kind: Kind::Other, msg: format!(
        "\"{:?}\" at file: \"{}\", line: {}", msg, location.file(), location.line()
    )}
}

#[track_caller]
pub fn parse_shader_error(msg: &str) -> Error {
    let location = std::panic::Location::caller();
    Error { kind: Kind::ParseShaderError, msg: format!(
        "\"{}\" at file: \"{}\", line: {}", msg, location.file(), location.line()
    )}
}


#[track_caller]
pub fn link_program_error(msg: &str) -> Error {
    let location = std::panic::Location::caller();
    Error { kind: Kind::LinkProgramError, msg: format!(
        "\"{}\" at file: \"{}\", line: {}", msg, location.file(), location.line()
    )}
}


#[derive(Debug, PartialEq)]
pub enum Kind {
    Other,
    ParseShaderError,
    LinkProgramError
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
