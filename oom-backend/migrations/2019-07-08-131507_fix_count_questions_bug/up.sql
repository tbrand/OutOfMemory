-- Your SQL goes here
DROP VIEW rank_users;

CREATE VIEW rank_users AS
  SELECT
    users.id AS id,
    users.name AS name,
    users.avatar_url AS avatar_url,
    CASE WHEN posts.id IS NULL THEN 0 ELSE IFNULL(SUM(ISNULL(posts.question_post_id)), 0) END AS count_questions,
    IFNULL(SUM(!ISNULL(posts.question_post_id)), 0) AS count_answers,
    IFNULL(SUM(posts.is_best_answer), 0) AS count_best_answers,
    users.created_at AS created_at,
    users.updated_at AS updated_at
    FROM users
    LEFT OUTER JOIN posts ON posts.user_id = users.id
    GROUP BY users.id
