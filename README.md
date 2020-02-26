# ![RealWorld Example App](logo.png)

An async Rust REST API backend using [Warp](https://github.com/seanmonstar/warp), [Diesel](https://github.com/diesel-rs/diesel), Postgres, and JWT that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

Includes real world examples for CRUD operations, authentication, routing, and pagination.

For more information on how this works with various frontends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

Originally forked from https://github.com/colinbankier/realworld-tide.

## Getting started

- Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with the `postgres` feature enabled.
- Start a postgres database by running `docker-compose up -d` (requires `docker-compose`) or use your own method.
- Copy [`.env.example`](./.env.example) to `.env` and change any environment variables accordingly to your system.
- Setup the database by running `diesel database setup`.
- When you are done, stop the database with `docker-compose stop`.

The URL of the API will be the value of `BIND_ADDRESS` in `.env` along with `/api`, e.g. `https://127.0.0.1:8088/api`.

To test the API, RealWorld provides a [Postman collection](https://github.com/gothinkster/realworld/blob/master/api/Conduit.postman_collection.json) that we can use.

To run the tests, do the following:

```bash
git clone https://github.com/gothinkster/realworld
cd realworld/api
APIURL=http://localhost:8088/api ./run-api-tests.sh
```

## Libraries used

- [chrono](https://github.com/chronotope/chrono)
- [diesel](https://github.com/diesel-rs/diesel)
- [dotenv](https://github.com/dotenv-rs/dotenv)
- [env_logger](https://github.com/sebasmagri/env_logger/)
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [r2d2](https://github.com/sfackler/r2d2)
- [rust-bcrypt](https://github.com/Keats/rust-bcrypt)
- [serde](https://github.com/serde-rs/serde)
- [tokio](https://github.com/tokio-rs/tokio)
- [uuid](https://github.com/uuid-rs/uuid)
- [warp](https://github.com/seanmonstar/warp)

## Note on session management

We use JWT to comply with the RealWorld specification, however in practice you should not use JWT for session management. Instead you should use persistent or session cookies with server-side session management.

For reference: [Stop Using JWT for sessions and why your solution doesn't work](http://cryto.net/~joepie91/blog/2016/06/19/stop-using-jwt-for-sessions-part-2-why-your-solution-doesnt-work/).

## Limitations

Currently, r2d2 and Diesel are both synchronous.
