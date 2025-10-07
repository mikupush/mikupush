// @generated automatically by Diesel CLI.

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
