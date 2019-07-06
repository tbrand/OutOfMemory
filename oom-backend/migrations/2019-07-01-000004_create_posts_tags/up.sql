-- Your SQL goes here
CREATE TABLE posts_tags(
  post_id INTEGER UNSIGNED NOT NULL,
  tag_id INTEGER UNSIGNED NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts(id) ON UPDATE CASCADE ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON UPDATE CASCADE ON DELETE CASCADE,
  PRIMARY KEY (post_id, tag_id)
)