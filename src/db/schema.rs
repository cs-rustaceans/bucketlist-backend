// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        role -> Varchar,
        email -> Varchar,
        password -> Varchar,
        status -> Varchar,
    }
}
