# test-rocket
Testing Rust Rocket framework

## Requirements

- Rust nightly `rustup default nightly`
- Node/npm

## Setup

- Copy example env `cp .env.example .env`
- Update `.env` database url to the appropriate value
- Run migrations `diesel migration run`
- Run asset build `npm run dev` (or `npm run prod`)
- Start app `cargo run`

#### Resources

- https://rocket.rs/v0.4/guide/getting-started/
- https://diesel.rs/guides/getting-started/
- https://github.com/luisvonmuller/heroes-crud-rust
