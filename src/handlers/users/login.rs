use super::responses::UserResponse;
use crate::app::AppState;
use crate::response::Response;
use serde::Deserialize;

use crate::auth::encode_token;
use domain::repositories::Repository;

#[derive(Deserialize)]
pub struct AuthRequest {
    user: AuthUser,
}

#[derive(Deserialize)]
pub struct AuthUser {
    email: String,
    password: String,
}

pub async fn login(form: AuthRequest, state: AppState) -> Response {
    let repository = &state.repository;

    let logged_in_user =
        repository.get_user_by_email_and_password(&form.user.email, &form.user.password)?;
    let token = encode_token(&state.jwt_secret, logged_in_user.id);

    let response = UserResponse::from((logged_in_user, token));

    Ok(warp::reply::json(&response))
}
