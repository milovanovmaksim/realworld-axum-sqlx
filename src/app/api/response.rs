use crate::app::domain::error::AppError;

pub type ApiResponse<T> = Result<T, AppError>;
