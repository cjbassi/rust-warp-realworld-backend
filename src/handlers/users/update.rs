use crate::app::AppState;
use crate::response::Response;
use domain;
use serde::{Deserialize, Serialize};

use crate::auth::{decode_token, encode_token};
use crate::handlers::users::responses::UserResponse;
use domain::repositories::Repository;
use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub user: UpdateUserRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl TryFrom<UpdateUserRequest> for domain::UserUpdate {
    type Error = domain::PasswordError;

    fn try_from(u: UpdateUserRequest) -> Result<Self, Self::Error> {
        let update = Self {
            email: u.email,
            username: u.username,
            password: u
                .password
                .map(domain::Password::from_clear_text)
                .transpose()?,
            image: u.image,
            bio: u.bio,
        };
        Ok(update)
    }
}

pub async fn update_user(token: String, form: UpdateUserRequest, state: AppState) -> Response {
    let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    let repository = &state.repository;

    let user = repository.get_user_by_id(user_id)?;
    let updated_user = user.update(form.try_into()?, repository)?;
    let token = encode_token(&state.jwt_secret, updated_user.id);

    let response = UserResponse::from((updated_user, token));

    Ok(warp::reply::json(&response))
}
