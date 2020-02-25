#[macro_use]
extern crate diesel;

pub mod connection;
pub mod models;
pub mod queries;
pub mod repository;
pub mod schema;
pub mod shims;

pub use repository::Repository;

pub use connection::Repo;
