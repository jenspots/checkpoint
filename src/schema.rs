// @generated automatically by Diesel CLI.

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
