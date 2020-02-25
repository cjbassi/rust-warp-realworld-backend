use crate::app::AppState;
use crate::auth::decode_token;
use crate::handlers::profiles::responses::ProfileResponse;
use crate::response::Response;
use domain::repositories::Repository;

pub async fn get_profile(username: String, token: Option<String>, state: AppState) -> Response {
    let user_id = token
        .map(|token| -> jsonwebtoken::errors::Result<uuid::Uuid> {
            Ok(decode_token(&state.jwt_secret, &token)?.user_id())
        })
        .transpose()?;
    let repository = &state.repository;

    let response: ProfileResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let view = repository.get_profile_view(&user, &username)?;
            ProfileResponse::from(view)
        }
        None => {
            let profile = repository.get_profile(&username)?;
            ProfileResponse::from(profile)
        }
    };

    Ok(warp::reply::json(&response))
}
