use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::articles::responses::ArticlesResponse;
use crate::response::Response;
use domain::repositories::Repository;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct ArticleQuery {
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub tag: Option<String>,
}

impl From<ArticleQuery> for domain::ArticleQuery {
    fn from(q: ArticleQuery) -> Self {
        Self {
            author: q.author,
            favorited: q.favorited,
            tag: q.tag,
        }
    }
}

pub async fn list_articles(
    token: Option<String>,
    query: ArticleQuery,
    state: AppState,
) -> Response {
    let repository = &state.repository;

    let user_id = token
        .map(|token| -> jsonwebtoken::errors::Result<uuid::Uuid> {
            Ok(decode_token(&state.jwt_secret, &token)?.user_id())
        })
        .transpose()?;
    let articles = repository.find_articles(query.into())?;
    let response: ArticlesResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let views = repository.get_articles_views(&user, articles)?;
            ArticlesResponse::from(views)
        }
        None => ArticlesResponse::from(articles),
    };
    Ok(warp::reply::json(&response))
}
