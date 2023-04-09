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

## DTO

The following DTO's are defined, to e used when interacting with the API provided by the application:

### User
```json
{
    "id": u64,
    "role": String,
    "email": String,
    "password": String
}
```

### NewUser
```json
{
    "role": String,
    "email": String,
    "password": String
}
```

### LoginForm
```json
{
    "email": String,
    "password": String
}
```

## API

The following endpoints are exposed by the application:

- `GET /admin/users`
  - Does not require anything in the Request body.
  - Returns a `List` containing all the users(`User`) of the application in the Response body. 

- `POST /admin/users`
  - Requires a `NewUser` in the Request body
  - Does not return anything in the Response body.

- `GET /admin/users/{id}`
  - Does not require anything in the Request body
  - Returns an `User` with the specified id in the Response body. 

- `UPDATE /admin/users/{id}`
  - Requires a `NewUser` in the Request body
  - Does not return anything in the Response body

- `DELETE /admin/users/{id}`
  - Does not require anything in the Request body
  - Does not return anything in the Response body

- `GET /login`
  - Requires a `LoginForm` in the Request body
  - Returns a `String` with the role of the specified user in the Response body.


