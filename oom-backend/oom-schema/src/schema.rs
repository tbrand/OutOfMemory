table! {
    comments (id) {
        id -> Unsigned<Integer>,
        body -> Text,
        user_id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Text,
        body -> Longtext,
        is_best_answer -> Bool,
        user_id -> Unsigned<Integer>,
        question_post_id -> Nullable<Unsigned<Integer>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts_tags (post_id, tag_id) {
        post_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        description -> Text,
        user_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    ups (user_id, post_id) {
        user_id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        is_up -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        avatar_url -> Varchar,
        token -> Nullable<Varchar>,
        token_expire -> Nullable<Timestamp>,
        access_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_tags (user_id, tag_id) {
        user_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    views (user_id, post_id) {
        user_id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));
joinable!(tags -> users (user_id));
joinable!(ups -> posts (post_id));
joinable!(ups -> users (user_id));
joinable!(users_tags -> tags (tag_id));
joinable!(users_tags -> users (user_id));
joinable!(views -> posts (post_id));
joinable!(views -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    posts_tags,
    tags,
    ups,
    users,
    users_tags,
    views,
);
