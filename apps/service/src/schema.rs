//TODO: assets table
diesel::table! {
    venues (id) {
        id -> Int4,
        owner_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        venue_logo -> Nullable<Text>,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        country -> Nullable<Text>,
        zip_code -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        email -> Nullable<Text>,
        website -> Nullable<Text>,
        social_media_links -> Nullable<Text>,
        map_url -> Nullable<Text>,
        embed_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        username -> Text,
        password -> Text,
        bio -> Nullable<Text>,
        google_id -> Nullable<Text>,
        profile_picture -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(venues -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(venues, users);
