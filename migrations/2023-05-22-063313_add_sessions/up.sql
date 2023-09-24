CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  userId BIGINT UNSIGNED NOT NULL,
  createdAt TIMESTAMP NOT NULL,
  expiresAt TIMESTAMP NOT NULL
);

ALTER TABLE sessions ADD FOREIGN KEY (userId) REFERENCES users(id) ON DELETE CASCADE;
