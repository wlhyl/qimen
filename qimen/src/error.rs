use std::{collections::HashMap, fmt::Display};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
    InvalidDateTime(String),
    InvalidZone(String),
    Function(String),
    // 无效的推运时间
    // InvalidProcessDateTime(String),
    BadRequest(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Function(s) => s,
            // Error::InvalidProcessDateTime(s) => s,
            Error::InvalidDateTime(s) => s,
            Error::InvalidZone(s) => s,
            Error::BadRequest(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidDateTime(_) | Error::InvalidZone(_) | Error::BadRequest(_) => {
                StatusCode::BAD_REQUEST
            }
            Error::Function(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut result = HashMap::new();
        result.insert("error", format!("{}", self));
        HttpResponse::build(self.status_code()).json(result)
    }
}

impl From<horo_date_time::Error> for Error {
    fn from(value: horo_date_time::Error) -> Self {
        match value {
            horo_date_time::Error::InvalidDateTime(s) => Self::InvalidDateTime(s),
            horo_date_time::Error::InvalidZone(s) => Self::InvalidZone(s),
            horo_date_time::Error::Function(s) => Self::Function(s),
        }
    }
}

impl From<actix_web_validator::Error> for Error {
    fn from(value: actix_web_validator::Error) -> Self {
        Error::BadRequest(value.to_string())
    }
}
