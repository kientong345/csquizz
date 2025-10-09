-- Add up migration script here

ALTER TABLE questions ADD COLUMN image_url VARCHAR(255);

ALTER TABLE user_answers ADD COLUMN text_answer VARCHAR(999);
