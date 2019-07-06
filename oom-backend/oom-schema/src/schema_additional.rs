use crate::schema::*;

table! {
    questions (id) {
        id -> Unsigned<Integer>,
        title -> Text,
        body -> Text,
        is_solved -> Bool,
        user_id -> Unsigned<Integer>,
        count_ups -> BigInt,
        count_downs -> BigInt,
        count_views -> BigInt,
        count_answers -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tag_questions (id) {
        id -> Unsigned<Integer>,
        title -> Text,
        body -> Text,
        is_solved -> Bool,
        user_id -> Unsigned<Integer>,
        count_ups -> BigInt,
        count_downs -> BigInt,
        count_views -> BigInt,
        count_answers -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    list_tags (id) {
        id -> Unsigned<Integer>,
        name -> Text,
        description -> Text,
        user_id -> Unsigned<Integer>,
        count_posts -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    answers (id) {
        id -> Unsigned<Integer>,
        title -> Text,
        body -> Text,
        is_best_answer -> Bool,
        user_id -> Unsigned<Integer>,
        question_post_id -> Unsigned<Integer>,
        count_ups -> BigInt,
        count_downs -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    rank_users (id) {
        id -> Unsigned<Integer>,
        name -> Text,
        avatar_url -> Text,
        count_questions -> BigInt,
        count_answers -> BigInt,
        count_best_answers -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(posts_tags -> questions (post_id));
joinable!(answers -> questions (question_post_id));
joinable!(users_tags -> list_tags (tag_id));

allow_tables_to_appear_in_same_query!(questions, posts_tags);
allow_tables_to_appear_in_same_query!(questions, answers);
allow_tables_to_appear_in_same_query!(list_tags, users_tags);
