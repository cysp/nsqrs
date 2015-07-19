use std;


#[derive(Debug)]
pub enum ErrorKind {
    UnknownFrameType(u32),
    InvalidMessageFrame,
    InvalidMessageId(Vec<u8>),
}

#[derive(Debug)]
pub enum Error {
    NsqError(ErrorKind),
    IoError(std::io::Error),
}

impl Error {
    pub fn unknown_frame_type_error(frame_type: u32) -> Error {
        Error::NsqError(ErrorKind::UnknownFrameType(frame_type))
    }

    pub fn invalid_message_frame_error() -> Error {
        Error::NsqError(ErrorKind::InvalidMessageFrame)
    }

    pub fn invalid_message_id_error(message_id: &[u8]) -> Error {
        Error::NsqError(ErrorKind::InvalidMessageId(message_id.to_owned()))
    }
}


impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NsqError(ref kind) => {
                match kind {
                    &ErrorKind::UnknownFrameType(_) => "unknown frame type",
                    &ErrorKind::InvalidMessageFrame => "invalid message frame",
                    &ErrorKind::InvalidMessageId(_) => "invalid message id",
                }
            }
            // ErrorRepr::WithDescription(_, desc) => desc,
            // ErrorRepr::WithDescriptionAndDetail(_, desc, _) => desc,
            // ErrorRepr::ExtensionError(_, _) => "extension error",
            Error::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::NsqError(_) => None,
            Error::IoError(ref err) => Some(err as &std::error::Error),
            // _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::NsqError(ref kind) => {
                match kind {
                    &ErrorKind::UnknownFrameType(ref frame_type) => write!(f, "Unknown frame type: ({})", frame_type),
                    &ErrorKind::InvalidMessageFrame => write!(f, "Invalid message frame"),
                    &ErrorKind::InvalidMessageId(ref message_id) => write!(f, "Invalid message id: ({:?})", message_id),
                }
            }
            Error::IoError(ref err) => err.fmt(f),
        }
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}
