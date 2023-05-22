// @generated automatically by Diesel CLI.

diesel::table! {
    destinations (id) {
        id -> Unsigned<Bigint>,
        ownerId -> Unsigned<Bigint>,
        visibility -> Varchar,
        isReviewed -> Bool,
        name -> Varchar,
        latitude -> Double,
        longitude -> Double,
    }
}

diesel::table! {
    sessions (id) {
        id -> Varchar,
        userId -> Nullable<Unsigned<Bigint>>,
        createdAt -> Nullable<Date>,
        expiresAt -> Nullable<Date>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        role -> Varchar,
        email -> Varchar,
        password -> Varchar,
        status -> Varchar,
    }
}

diesel::joinable!(destinations -> users (ownerId));
diesel::joinable!(sessions -> users (userId));

diesel::allow_tables_to_appear_in_same_query!(destinations, sessions, users,);
