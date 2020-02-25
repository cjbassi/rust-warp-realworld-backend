//! A sub-module to prescribe how each domain error gets converted to an HTTP response.

use jsonwebtoken::errors::Error as JwtError;
use warp::http::response::Response;
use warp::http::status::StatusCode;
use warp::reply::with_status;
use warp::reply::Reply;

use domain::{
    ChangeArticleError, DatabaseError, DeleteCommentError, GetArticleError, GetUserError,
    LoginError, PasswordError, PublishArticleError, SignUpError,
};

use crate::response::ErrorResponse;

impl From<JwtError> for ErrorResponse {
    fn from(_: JwtError) -> ErrorResponse {
        ErrorResponse(StatusCode::UNAUTHORIZED.into_response())
    }
}

impl From<GetUserError> for ErrorResponse {
    fn from(e: GetUserError) -> ErrorResponse {
        match &e {
            GetUserError::NotFound { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::NOT_FOUND).into_response(),
            ),
            GetUserError::DatabaseError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}

impl From<PasswordError> for ErrorResponse {
    fn from(_: PasswordError) -> ErrorResponse {
        ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

impl From<LoginError> for ErrorResponse {
    fn from(e: LoginError) -> ErrorResponse {
        log::debug!("{}\n", "hello");
        let r = match &e {
            LoginError::NotFound => StatusCode::UNAUTHORIZED,
            LoginError::PasswordError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ErrorResponse(r.into_response())
    }
}

impl From<SignUpError> for ErrorResponse {
    fn from(e: SignUpError) -> ErrorResponse {
        let r = match &e {
            SignUpError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ErrorResponse(r.into_response())
    }
}

impl From<GetArticleError> for ErrorResponse {
    fn from(e: GetArticleError) -> ErrorResponse {
        match &e {
            GetArticleError::ArticleNotFound { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::NOT_FOUND).into_response(),
            ),
            GetArticleError::DatabaseError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}

impl From<DatabaseError> for ErrorResponse {
    fn from(_: DatabaseError) -> ErrorResponse {
        ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

impl From<PublishArticleError> for ErrorResponse {
    fn from(e: PublishArticleError) -> ErrorResponse {
        match &e {
            PublishArticleError::AuthorNotFound { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::NOT_FOUND).into_response(),
            ),
            PublishArticleError::DuplicatedSlug { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::BAD_REQUEST).into_response(),
            ),
            PublishArticleError::DatabaseError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}

impl From<ChangeArticleError> for ErrorResponse {
    fn from(e: ChangeArticleError) -> ErrorResponse {
        match &e {
            ChangeArticleError::ArticleNotFound { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::NOT_FOUND).into_response(),
            ),
            ChangeArticleError::Forbidden { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::UNAUTHORIZED).into_response(),
            ),
            ChangeArticleError::DatabaseError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}

impl From<DeleteCommentError> for ErrorResponse {
    fn from(e: DeleteCommentError) -> ErrorResponse {
        match &e {
            DeleteCommentError::CommentNotFound { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::NOT_FOUND).into_response(),
            ),
            DeleteCommentError::Forbidden { .. } => ErrorResponse(
                with_status(Response::new(e.to_string()), StatusCode::UNAUTHORIZED).into_response(),
            ),
            DeleteCommentError::DatabaseError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}
