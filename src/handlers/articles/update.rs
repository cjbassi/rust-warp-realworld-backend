use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::articles::responses::ArticleResponse;
use crate::response::Response;
use domain::repositories::Repository;
use domain::ArticleUpdate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: UpdateArticleRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

impl From<Request> for ArticleUpdate {
    fn from(r: Request) -> ArticleUpdate {
        ArticleUpdate {
            title: r.article.title,
            body: r.article.body,
            description: r.article.description,
        }
    }
}

pub async fn update_article(
    slug: String,
    token: String,
    form: Request,
    state: AppState,
) -> Response {
    let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    let repository = &state.repository;

    let article = repository.get_article_by_slug(&slug)?;
    let user = repository.get_user_by_id(user_id)?;
    let updated_article = user.update_article(article, form.into(), repository)?;

    let response: ArticleResponse = repository.get_article_view(&user, updated_article)?.into();
    Ok(warp::reply::json(&response))
}
