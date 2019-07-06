-- Your SQL goes here
CREATE VIEW answers AS
  SELECT
    posts.id AS id,
    posts.title AS title,
    posts.body AS body,
    posts.is_best_answer AS is_best_answer,
    posts.user_id AS user_id,
    posts.question_post_id AS question_post_id,
    (SELECT COUNT(up_ups.post_id) FROM ups AS up_ups
      WHERE up_ups.post_id = posts.id AND up_ups.is_up IS TRUE) AS count_ups,
    (SELECT COUNT(down_ups.post_id) FROM ups AS down_ups
      WHERE down_ups.post_id = posts.id AND down_ups.is_up IS FALSE) AS count_downs,
    posts.created_at AS created_at,
    posts.updated_at AS updated_at
    FROM posts WHERE posts.question_post_id IS NOT NULL
