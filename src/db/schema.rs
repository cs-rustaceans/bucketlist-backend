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
    users (id) {
        id -> Unsigned<Bigint>,
        role -> Varchar,
        email -> Varchar,
        password -> Varchar,
        status -> Varchar,
    }
}

diesel::joinable!(destinations -> users (ownerId));

diesel::allow_tables_to_appear_in_same_query!(
    destinations,
    users,
);
