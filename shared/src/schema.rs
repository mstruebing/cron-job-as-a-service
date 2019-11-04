table! {
    jobs (id) {
        id -> Int4,
        user_id -> Int4,
        schedule -> Varchar,
        command -> Varchar,
        last_run -> Int4,
        next_run -> Int4,
    }
}

table! {
    secrets (id) {
        id -> Int4,
        job_id -> Int4,
        key -> Text,
        value -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
    }
}

joinable!(jobs -> users (user_id));
joinable!(secrets -> jobs (job_id));

allow_tables_to_appear_in_same_query!(
    jobs,
    secrets,
    users,
);
