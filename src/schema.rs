table! {
    buckets (bucket_id) {
        bucket_id -> Int4,
        name -> Varchar,
        date_created -> Timestamp,
        date_modified -> Timestamp,
        user_id -> Nullable<Int4>,
    }
}

table! {
    items (item_id) {
        item_id -> Int4,
        name -> Varchar,
        bucket_id -> Nullable<Int4>,
        date_created -> Timestamp,
        date_modified -> Timestamp,
        completed -> Nullable<Bool>,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        name -> Varchar,
        password -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    buckets,
    items,
    users,
);
