table! {
    registrations (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        resort_id -> Int4,
    }
}

table! {
    resorts (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    registrations,
    resorts,
);
