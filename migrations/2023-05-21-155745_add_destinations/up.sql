-- Your SQL goes here
CREATE TABLE destinations (
  id SERIAL PRIMARY KEY,
  ownerId BIGINT UNSIGNED NOT NULL, 
  visibility VARCHAR(255) NOT NULL,
  isReviewed BOOLEAN NOT NULL,
  name VARCHAR(255) NOT NULL,
  latitude DOUBLE NOT NULL,
  longitude DOUBLE NOT NULL
);

ALTER TABLE destinations ADD FOREIGN KEY (ownerId) REFERENCES users(id);
