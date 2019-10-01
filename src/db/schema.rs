table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hash -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
    }
}
