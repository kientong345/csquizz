-- Add up migration script here
CREATE TYPE user_role AS ENUM (
    'user',
    'admin'
);

CREATE TYPE quiz_difficulty AS ENUM (
    'easy',
    'medium',
    'hard'
);

CREATE TYPE question_form AS ENUM (
    'multiple-choice',
    'single-choice',
    'text-entry'
);

ALTER TABLE users
    ALTER COLUMN role DROP DEFAULT,
    ALTER COLUMN role TYPE user_role USING role::text::user_role,
    ALTER COLUMN role SET NOT NULL,
    ALTER COLUMN role SET DEFAULT 'user';

ALTER TABLE quizzes
    ALTER COLUMN difficulty TYPE quiz_difficulty USING difficulty::text::quiz_difficulty;

ALTER TABLE questions
    ALTER COLUMN question_type TYPE question_form USING question_type::text::question_form;
