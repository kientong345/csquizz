-- Add down migration script here

ALTER TABLE user_answers DROP COLUMN entried_text;

ALTER TABLE questions DROP COLUMN correct_entry;

ALTER TABLE questions DROP COLUMN image_url;
