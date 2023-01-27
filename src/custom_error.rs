pub struct Error{
    pub error_type: String,
    pub message: String
}

impl Error {
    pub fn new(message: String, error_type: String) -> Error {
        return Error { message: message, error_type: error_type };
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", &self.error_type, &self.message)
    }
}