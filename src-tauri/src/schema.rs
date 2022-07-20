table! {
    users (id) {
        id -> Nullable<Integer>,
        chat_id -> Text,
        passwd -> Text,
        email -> Text,
        phone -> Nullable<Text>,
        name -> Text,
        profile_image -> Binary,
        sex -> Integer,
        region -> Text,
        personalized_signature -> Nullable<Text>,
    }
}
