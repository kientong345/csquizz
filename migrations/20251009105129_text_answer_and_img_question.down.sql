-- Add down migration script here

ALTER TABLE user_answers DROP COLUMN text_answer;

ALTER TABLE questions DROP COLUMN correct_text_answer;

ALTER TABLE questions DROP COLUMN image_url;
