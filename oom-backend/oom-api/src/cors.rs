use warp::http::StatusCode;

pub fn cors() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(StatusCode::OK)
}
