pub enum ReaderError {
    SensorNotFound(String),
    ParseError(String),
    ReadingError(String),
}

impl std::fmt::Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::SensorNotFound(b) => write!(f, "{b} sensor not found!"),
            ReaderError::ParseError(b) => write!(f, "Couldn't parse {b}"),
            ReaderError::ReadingError(b) => write!(f, "Error reading {b}"),
        }
    }
}
