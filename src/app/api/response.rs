use crate::app::error::AppError;

pub type ApiResponse<T> = Result<T, AppError>;
