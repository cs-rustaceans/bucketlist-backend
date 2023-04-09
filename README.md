## Deployment

Required tools:
- cargo
  - `https://doc.rust-lang.org/cargo/getting-started/installation.html`
- diesel-cli
  - `cargo install diesel_cli`

For deployment, the following steps must be followed:
  - Provide `DATABASE_URL` either through `.env` or environmental variables. It should have the format specified in `.env-example`
  - [OPTIONAL] Provide `PORT` either through `.env` or environmental variables. By default, the application will run on port `8080`
  - Run `diesel migration run`
  - Run `cargo run`


