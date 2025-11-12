-- Add up migration script here

ALTER TABLE questions ADD COLUMN image_url VARCHAR(255);

ALTER TABLE questions ADD COLUMN correct_entry VARCHAR(999);

ALTER TABLE user_answers ADD COLUMN entried_text VARCHAR(999);
