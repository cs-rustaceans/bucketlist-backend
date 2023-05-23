CREATE TABLE bucketlist_items (
  id SERIAL PRIMARY KEY,
  destinationId BIGINT UNSIGNED NOT NULL,
  ownerId BIGINT UNSIGNED NOT NULL,
  startDate DATE NOT NULL,
  endDate DATE NOT NULL
);

ALTER TABLE bucketlist_items ADD FOREIGN KEY (ownerId) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE bucketlist_items ADD FOREIGN KEY (destinationId) REFERENCES destinations(id) ON DELETE CASCADE;

-- Your SQL goes here
