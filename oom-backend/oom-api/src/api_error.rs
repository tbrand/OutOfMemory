use std::error::Error as StdError;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Debug)]
pub struct ApiError {
    message: String,
    status_code: StatusCode,
}

#[derive(Serialize, Debug)]
pub struct ApiErrorResponse {
    message: String,
}

impl ApiError {
    pub fn new(message: String, status_code: StatusCode) -> ApiError {
        ApiError {
            message,
            status_code,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&format!("{} ({})", self.message, self.status_code))
    }
}

impl StdError for ApiError {}

impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> ApiError {
        ApiError::new(
            format!("database error={:?}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

pub fn api_error(message: &str, status_code: StatusCode) -> warp::Rejection {
    warp::reject::custom(ApiError::new(message.to_owned(), status_code))
}

pub fn as_reply(e: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = e.find_cause::<ApiError>() {
        let json = warp::reply::json(&ApiErrorResponse {
            message: e.message.clone(),
        });

        Ok(warp::reply::with_status(json, e.status_code))
    } else {
        Err(e)
    }
}
