use std::env;
use std::net::SocketAddr;

use warp::{self, Filter};

use crate::routes;
use db::{connection::Repo, Repository};

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct AppState {
    pub repository: Repository,
    pub jwt_secret: String,
}

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let repository = Repository(Repo::new(&database_url));

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = AppState {
        repository,
        jwt_secret,
    };

    let routes = routes::routes(app_state).with(warp::log(APPLICATION_NAME));

    println!("You can access the server at {}", bind_address);

    warp::serve(routes).run(bind_address).await;
}
