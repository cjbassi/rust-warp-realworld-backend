use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::articles::responses::ArticleResponse;
use crate::response::Response;
use domain::repositories::Repository;

pub async fn favorite(slug: String, token: String, state: AppState) -> Response {
    _favorite(slug, token, state, Action::Favorite).await
}

pub async fn unfavorite(slug: String, token: String, state: AppState) -> Response {
    _favorite(slug, token, state, Action::Unfavorite).await
}

pub enum Action {
    Favorite,
    Unfavorite,
}

pub async fn _favorite(slug: String, token: String, state: AppState, action: Action) -> Response {
    let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    let repository = &state.repository;

    let user = repository.get_user_by_id(user_id)?;
    let article = repository.get_article_by_slug(&slug)?;
    let article_view = match action {
        Action::Favorite => user.favorite(article, repository),
        Action::Unfavorite => user.unfavorite(article, repository),
    }?;

    let response: ArticleResponse = article_view.into();
    Ok(warp::reply::json(&response))
}
