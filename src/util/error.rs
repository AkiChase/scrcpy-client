use std::fmt;

pub struct AppError {
    pub type_name: String,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR! {}: {}", self.type_name, self.message)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{type_name: {}, message: {} }}",
            self.type_name, self.message
        )
    }
}
