// We need this in order to compile although I'm not totally sure why.
// It has something to do with with the routes.
#![type_length_limit = "1422483"]

mod app;
mod auth;
mod errors;
mod handlers;
mod response;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    app::start().await;
}
