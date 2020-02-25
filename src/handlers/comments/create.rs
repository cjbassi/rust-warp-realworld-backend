use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::comments::responses::CommentResponse;
use crate::response::Response;
use domain::repositories::Repository;
use domain::CommentContent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub comment: NewCommentRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewCommentRequest {
    pub body: String,
}

pub async fn create(slug: String, token: String, form: Request, state: AppState) -> Response {
    let author_id = decode_token(&state.jwt_secret, &token)?.user_id();
    let repository = &state.repository;

    let author = repository.get_user_by_id(author_id)?;
    let article = repository.get_article_by_slug(&slug)?;
    let posted_comment = author.comment(&article, CommentContent(form.comment.body), repository)?;

    let response = CommentResponse {
        comment: posted_comment.into(),
    };
    Ok(warp::reply::json(&response))
}
