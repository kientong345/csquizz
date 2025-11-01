
CREATE TABLE users (
    id              SERIAL PRIMARY KEY,
    google_id       VARCHAR(255) UNIQUE,
    display_name    VARCHAR(50) NOT NULL,
    email           VARCHAR(100) UNIQUE NOT NULL,
    password_hash   TEXT,
    avatar_url      VARCHAR(255),
    role            VARCHAR(20) DEFAULT 'user', -- 'user' or 'admin'
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE categories (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(50) NOT NULL,  -- Data Structures, Algorithms, OS...
    image_url       VARCHAR(255),
    description     TEXT
);

CREATE TABLE quizzes (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    description     TEXT,
    category        INT REFERENCES categories(id) ON DELETE CASCADE,
    difficulty      VARCHAR(20),  -- easy, medium, hard
    created_by      INT REFERENCES users(id) ON DELETE SET NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE questions (
    id              SERIAL PRIMARY KEY,
    quiz_id         INT REFERENCES quizzes(id) ON DELETE CASCADE,
	question_type	TEXT NOT NULL, -- multiple-choice, single-choice, text-entry ...
    question_text   TEXT NOT NULL,
    explanation     TEXT
);

CREATE TABLE options (
    id              SERIAL PRIMARY KEY,
    question_id     INT REFERENCES questions(id) ON DELETE CASCADE,
    option_text     TEXT NOT NULL,
    is_correct      BOOLEAN DEFAULT FALSE
);

CREATE TABLE results (
    id              SERIAL PRIMARY KEY,
    user_id         INT REFERENCES users(id) ON DELETE CASCADE,
    quiz_id         INT REFERENCES quizzes(id) ON DELETE CASCADE,
	score			FLOAT NOT NULL,
    total_questions INT NOT NULL,
    correct_answers INT NOT NULL,
    submitted_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_answers (
    id              SERIAL PRIMARY KEY,
    result_id       INT REFERENCES results(id) ON DELETE CASCADE,
    question_id     INT REFERENCES questions(id) ON DELETE CASCADE,
    selected_option INT REFERENCES options(id) ON DELETE SET NULL,
    is_correct      BOOLEAN NOT NULL
);