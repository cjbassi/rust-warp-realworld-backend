use warp;

pub type Response = std::result::Result<warp::reply::Json, ErrorResponse>;

pub struct ErrorResponse(pub warp::reply::Response);

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        self.0
    }
}
