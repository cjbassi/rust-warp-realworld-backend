use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::articles::responses::ArticlesResponse;
use crate::response::Response;
use domain::repositories::Repository;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FeedQuery {
    #[serde(default)]
    pub limit: u64,

    #[serde(default)]
    pub offset: u64,
}

impl Default for FeedQuery {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
        }
    }
}

impl From<FeedQuery> for domain::FeedQuery {
    fn from(f: FeedQuery) -> Self {
        Self {
            limit: f.limit,
            offset: f.offset,
        }
    }
}

pub async fn feed(token: String, query: FeedQuery, state: AppState) -> Response {
    let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    let repository = &state.repository;
    let user = repository.get_user_by_id(user_id)?;

    let articles = user.feed(query.into(), repository)?;
    let response = ArticlesResponse::from(articles);
    Ok(warp::reply::json(&response))
}
