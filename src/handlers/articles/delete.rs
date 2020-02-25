use crate::app::AppState;
use crate::auth::decode_token;
use crate::response::ErrorResponse;
use domain::repositories::Repository;

pub async fn delete_article(
    slug: String,
    token: String,
    state: AppState,
) -> Result<impl warp::reply::Reply, ErrorResponse> {
    let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    let repository = &state.repository;

    let user = repository.get_user_by_id(user_id)?;
    let article = repository.get_article_by_slug(&slug)?;
    user.delete(article, repository)?;

    Ok(warp::http::status::StatusCode::OK)
}
