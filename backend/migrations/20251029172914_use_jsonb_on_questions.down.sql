-- Add down migration script here
CREATE TABLE IF NOT EXISTS options (
    id              SERIAL PRIMARY KEY,
    question_id     INT REFERENCES questions(id) ON DELETE CASCADE,
    option_text     TEXT NOT NULL,
    is_correct      BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_options_question_id ON options (question_id);

ALTER TABLE user_answers DROP COLUMN answer_data;

ALTER TABLE user_answers ADD COLUMN is_correct BOOLEAN NOT NULL;

ALTER TABLE user_answers ADD COLUMN entried_text VARCHAR(999);

ALTER TABLE user_answers ADD COLUMN selected_option INT REFERENCES options(id) ON DELETE SET NULL;

CREATE INDEX idx_answers_selected_option ON user_answers (selected_option);

ALTER TABLE questions DROP COLUMN answer_key;

ALTER TABLE questions ADD COLUMN explanation TEXT;

ALTER TABLE questions ADD COLUMN correct_entry VARCHAR(999);