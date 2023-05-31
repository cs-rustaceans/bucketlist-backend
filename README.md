The backend of our Software Engineering's laboratory Bucketlist Application.
Check the wiki for more details(https://github.com/cs-rustaceans/bucketlist-wiki/wiki).
The wiki should be especially helpful, since it contains useful diagrams.

Written in rust using actix web, and diesel for migrations. The supported databse is MySQL.
This backend features role-protected routes, login using JWT tokens, CRUD for all entities, and transactions to make everything concurrency-safe.

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

