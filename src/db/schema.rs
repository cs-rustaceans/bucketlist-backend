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
        #[max_length = 255]
        visibility -> Varchar,
        isReviewed -> Bool,
        #[max_length = 255]
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
        #[max_length = 255]
        role -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
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
