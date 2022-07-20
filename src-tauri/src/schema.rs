table! {
    users (id) {
        id -> Integer,
        chat_id -> Text,
        passwd -> Text,
        email -> Text,
        phone -> Nullable<Text>,
        name -> Text,
        profile_image -> Nullable<Binary>,
        sex -> Integer,
        region -> Text,
        personalized_signature -> Nullable<Text>,
    }
}
