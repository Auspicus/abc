table! {
    experiments (id) {
        id -> Text,
        variants -> Integer,
    }
}

table! {
    sessions (id) {
        id -> Text,
        experiment_id -> Text,
        variant -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    experiments,
    sessions,
);
