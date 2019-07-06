-- Your SQL goes here
CREATE VIEW questions AS
  SELECT
    posts.id AS id,
    posts.title AS title,
    posts.body AS body,
    posts.user_id AS user_id,
    (SELECT COUNT(up_ups.post_id) FROM ups AS up_ups
      WHERE up_ups.post_id = posts.id AND up_ups.is_up IS TRUE) AS count_ups,
    (SELECT COUNT(down_ups.post_id) FROM ups AS down_ups
      WHERE down_ups.post_id = posts.id AND down_ups.is_up IS FALSE) AS count_downs,
    (SELECT COUNT(views.post_id) FROM views
      WHERE views.post_id = posts.id) AS count_views,
    (SELECT COUNT(answers.id) FROM posts AS answers
      WHERE answers.question_post_id = posts.id) AS count_answers,
    (SELECT COUNT(*) = 1 FROM posts AS best_answers
      WHERE best_answers.question_post_id = posts.id and best_answers.is_best_answer = true) as is_solved,
    posts.created_at AS created_at,
    posts.updated_at AS updated_at
    FROM posts WHERE posts.question_post_id IS NULL
