table! {
    users (id) {
        id -> Int8,
        created_at -> Timestamp,
        email -> Varchar,
        hash_pass -> Varchar,
    }
}
