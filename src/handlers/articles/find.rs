use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::articles::responses::ArticleResponse;
use crate::response::Response;
use domain::repositories::Repository;

pub async fn get_article(slug: String, token: Option<String>, state: AppState) -> Response {
    let repository = &state.repository;

    let article = repository.get_article_by_slug(&slug)?;
    let user_id = token
        .map(|token| -> jsonwebtoken::errors::Result<uuid::Uuid> {
            Ok(decode_token(&state.jwt_secret, &token)?.user_id())
        })
        .transpose()?;
    let response: ArticleResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id).unwrap();
            let article_view = repository.get_article_view(&user, article).unwrap();
            article_view.into()
        }
        None => article.into(),
    };
    Ok(warp::reply::json(&response))
}
