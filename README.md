# test-rocket
Testing Rust Rocket framework

## Setup

- Install rust
- Run nightly `rustup default nightly`
- Copy example env `cp .env.example .env`
- Update `.env` database url to the appropriate value
- Start app `cargo run`
- Run migrations `diesel migration run`

#### Resources

- https://rocket.rs/v0.4/guide/getting-started/
- https://diesel.rs/guides/getting-started/
- https://github.com/luisvonmuller/heroes-crud-rust
