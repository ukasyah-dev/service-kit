#[derive(Debug, thiserror::Error)]
#[error("Error {code}: {message}")]
pub struct Error {
    pub code: u16,
    pub message: String,
}

pub enum Code {
    AlreadyExists = 409,
    Internal = 500,
    InvalidArgument = 400,
    NotFound = 404,
    PermissionDenied = 403,
    Unauthenticated = 401,
}

/// Returns an `Error` object with the `Code::AlreadyExists` code.
///
/// If `msg` is empty, "Already exists" is used.
pub fn already_exists(msg: &str) -> Error {
    let mut message = msg.to_string();

    if message.is_empty() {
        message = "Already exists".to_string();
    }

    Error {
        code: Code::AlreadyExists as u16,
        message,
    }
}

/// Returns an `Error` object with the `Code::Internal` code.
///
/// If `msg` is empty, "Internal" is used.
pub fn internal(msg: &str) -> Error {
    let mut message = msg.to_string();

    if message.is_empty() {
        message = "Internal".to_string();
    }

    Error {
        code: Code::Internal as u16,
        message,
    }
}

/// Returns an `Error` object with the `Code::InvalidArgument` code.
///
/// If `msg` is empty, "Invalid argument" is used.
pub fn invalid_argument(msg: &str) -> Error {
    let mut message = msg.to_string();

    if message.is_empty() {
        message = "Invalid argument".to_string();
    }

    Error {
        code: Code::InvalidArgument as u16,
        message,
    }
}

/// Returns an `Error` object with the `Code::NotFound` code.
///
/// If `msg` is empty, "Not found" is used.
pub fn not_found(msg: &str) -> Error {
    let mut message = msg.to_string();

    if message.is_empty() {
        message = "Not found".to_string();
    }

    Error {
        code: Code::NotFound as u16,
        message,
    }
}

/// Returns an `Error` object with the `Code::PermissionDenied` code.
///
/// If `msg` is empty, "Permission denied" is used.
pub fn permission_denied(msg: &str) -> Error {
    let mut message = msg.to_string();

    if message.is_empty() {
        message = "Permission denied".to_string();
    }

    Error {
        code: Code::PermissionDenied as u16,
        message,
    }
}

/// Returns an `Error` object with the `Code::Unauthenticated` code.
///
/// If `msg` is empty, "Unauthenticated" is used.
pub fn unauthenticated(msg: &str) -> Error {
    let mut message = msg.to_string();

    if message.is_empty() {
        message = "Unauthenticated".to_string();
    }

    Error {
        code: Code::Unauthenticated as u16,
        message,
    }
}
