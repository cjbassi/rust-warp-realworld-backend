use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::comments::responses::CommentsResponse;
use crate::response::Response;
use domain::repositories::Repository;

pub async fn get(slug: String, token: Option<String>, state: AppState) -> Response {
    let user_id = token
        .map(|token| -> jsonwebtoken::errors::Result<uuid::Uuid> {
            Ok(decode_token(&state.jwt_secret, &token)?.user_id())
        })
        .transpose()?;
    let repository = &state.repository;

    let article = repository.get_article_by_slug(&slug)?;
    let comments = article.comments(repository)?;

    let response: CommentsResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let result: Result<Vec<_>, _> = comments
                .into_iter()
                .map(|c| c.view(&user, repository))
                .collect();
            let comment_views = result?;
            CommentsResponse::from(comment_views)
        }
        None => CommentsResponse::from(comments),
    };

    Ok(warp::reply::json(&response))
}
