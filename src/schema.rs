table! {
    apps (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        owner -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        token_lifetime -> Int4,
        domains -> Array<Text>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        profile_pic -> Nullable<Text>,
        email -> Text,
        verified -> Nullable<Bool>,
        created_at -> Timestamp,
        admin -> Bool,
        scopes -> Array<Text>,
        totp_token -> Nullable<Text>,
    }
}

joinable!(apps -> users (owner));

allow_tables_to_appear_in_same_query!(apps, users,);
