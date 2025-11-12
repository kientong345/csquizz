-- Add down migration script here
ALTER TABLE questions
    ALTER COLUMN question_type TYPE TEXT;

ALTER TABLE quizzes
    ALTER COLUMN difficulty TYPE VARCHAR(20);

ALTER TABLE users
    ALTER COLUMN role DROP DEFAULT,
    ALTER COLUMN role TYPE VARCHAR(20),
    ALTER COLUMN role DROP NOT NULL,
    ALTER COLUMN role SET DEFAULT 'user';

DROP TYPE question_form;

DROP TYPE quiz_difficulty;

DROP TYPE user_role;