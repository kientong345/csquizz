-- Add up migration script here
ALTER TABLE questions DROP COLUMN correct_entry;

ALTER TABLE questions DROP COLUMN explanation;

ALTER TABLE questions ADD COLUMN answer_key JSONB NOT NULL;

DROP INDEX idx_answers_selected_option;

ALTER TABLE user_answers DROP COLUMN selected_option;

ALTER TABLE user_answers DROP COLUMN entried_text;

ALTER TABLE user_answers DROP COLUMN is_correct;

ALTER TABLE user_answers ADD COLUMN answer_data JSONB NOT NULL;

DROP INDEX idx_options_question_id;

DROP TABLE options;
