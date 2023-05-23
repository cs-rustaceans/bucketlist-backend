// @generated automatically by Diesel CLI.

diesel::table! {
    bucketlist_items (id) {
        id -> Unsigned<Bigint>,
        destinationId -> Unsigned<Bigint>,
        ownerId -> Unsigned<Bigint>,
        startDate -> Timestamp,
        endDate -> Timestamp,
        isFavorite -> Bool,
    }
}

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
        id -> Unsigned<Bigint>,
        userId -> Unsigned<Bigint>,
        createdAt -> Timestamp,
        expiresAt -> Timestamp,
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

diesel::joinable!(bucketlist_items -> destinations (destinationId));
diesel::joinable!(bucketlist_items -> users (ownerId));
diesel::joinable!(destinations -> users (ownerId));
diesel::joinable!(sessions -> users (userId));

diesel::allow_tables_to_appear_in_same_query!(
    bucketlist_items,
    destinations,
    sessions,
    users,
);
