// @generated automatically by Diesel CLI.

diesel::table! {
    refresh_chain (id) {
        id -> Integer,
        user -> Integer,
        revoked_at -> Nullable<Integer>,
        created_at -> Integer,
        updated_at -> Nullable<Integer>,
    }
}

diesel::table! {
    refresh_token (id) {
        id -> Integer,
        used_at -> Nullable<Integer>,
        refresh_chain -> Integer,
        created_at -> Integer,
        updated_at -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        hash -> Binary,
        salt -> Binary,
        created_at -> Integer,
        updated_at -> Nullable<Integer>,
    }
}

diesel::joinable!(refresh_chain -> users (user));
diesel::joinable!(refresh_token -> refresh_chain (refresh_chain));

diesel::allow_tables_to_appear_in_same_query!(
    refresh_chain,
    refresh_token,
    users,
);
