use std::error::Error as StdError;

pub trait AppException: StdError {}

pub trait DomainException: AppException {}

impl<T: StdError> AppException for T {}
