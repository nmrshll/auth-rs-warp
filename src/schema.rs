table! {
    addresses (id) {
        id -> Int8,
        created_at -> Timestamp,
        label -> Varchar,
        pubkey -> Varchar,
        privkey -> Varchar,
        seed_phrase -> Varchar,
        owner_id -> Int8,
        currency_id -> Int8,
    }
}

table! {
    currencies (id) {
        id -> Int8,
        created_at -> Timestamp,
        name -> Nullable<Varchar>,
        symbol -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Int8,
        created_at -> Timestamp,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        author_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        created_at -> Timestamp,
        email -> Varchar,
        hash_pass -> Varchar,
    }
}

joinable!(addresses -> currencies (currency_id));
joinable!(addresses -> users (owner_id));
joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    currencies,
    posts,
    users,
);
