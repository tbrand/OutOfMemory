-- Your SQL goes here
CREATE VIEW list_tags AS
  SELECT
    tags.id AS id,
    tags.name AS name,
    tags.description AS description,
    tags.user_id AS user_id,
    (SELECT COUNT(posts.id)) AS count_posts,
    tags.created_at AS created_at,
    tags.updated_at AS updated_at
    FROM tags
    LEFT OUTER JOIN posts_tags ON posts_tags.tag_id = tags.id
    LEFT OUTER JOIN posts ON posts_tags.post_id = posts.id
    GROUP BY tags.id
