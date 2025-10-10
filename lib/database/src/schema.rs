// @generated automatically by Diesel CLI.

diesel::table! {
    config (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    servers (id) {
        id -> Text,
        url -> Text,
        name -> Text,
        icon -> Nullable<Text>,
        alias -> Nullable<Text>,
        added_at -> Timestamp,
        testing -> Bool,
        connected -> Bool,
        healthy -> Bool,
    }
}

diesel::table! {
    uploads (id) {
        id -> Text,
        name -> Text,
        size -> BigInt,
        mime_type -> Text,
        path -> Text,
        url -> Text,
        server_id -> Text,
        created_at -> Timestamp,
        status -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    config,
    servers,
    uploads,
);
