-- Add migration script here

-- ENUM types
CREATE TYPE user_role AS ENUM ('user', 'admin');
CREATE TYPE quiz_difficulty AS ENUM ('easy', 'medium', 'hard');
CREATE TYPE question_type AS ENUM ('single_choice', 'multiple_choice', 'text_entry');

-- USERS
CREATE TABLE IF NOT EXISTS users (
    usr_id              SERIAL PRIMARY KEY,
    usr_google_id       TEXT UNIQUE,
    usr_display_name    VARCHAR(50) NOT NULL,
    usr_email           VARCHAR(100) UNIQUE NOT NULL,
    usr_password_hash   TEXT,
    usr_avatar_url      TEXT,
    usr_role            user_role NOT NULL DEFAULT 'user',
    usr_created_at      TIMESTAMPTZ DEFAULT NOW()
);

-- CATEGORIES
CREATE TABLE IF NOT EXISTS categories (
    cat_id              SERIAL PRIMARY KEY,
    cat_name            VARCHAR(50) NOT NULL UNIQUE,
    cat_image_url       TEXT,
    cat_description     TEXT
);

-- QUIZZES
CREATE TABLE IF NOT EXISTS quizzes (
    qz_id              	SERIAL PRIMARY KEY,
    qz_title           	VARCHAR(200) NOT NULL,
    qz_description     	TEXT,
    qz_difficulty      	quiz_difficulty NOT NULL,
    qz_category_id     	INT REFERENCES categories(cat_id) ON DELETE SET NULL,
    qz_creator_id      	INT REFERENCES users(usr_id) ON DELETE SET NULL,
    qz_pass_score       FLOAT NOT NULL CHECK (qz_pass_score >= 0),
    qz_created_at      	TIMESTAMPTZ DEFAULT NOW(),
    qz_updated_at      	TIMESTAMPTZ DEFAULT NOW()
);

-- QUESTIONS
CREATE TABLE IF NOT EXISTS questions (
    qs_id              	SERIAL PRIMARY KEY,
    qs_type          	  question_type NOT NULL,
    qs_content         	TEXT NOT NULL,
    qs_image_url       	TEXT,
    qs_key	        	  JSONB NOT NULL,
    qs_quiz_id         	INT REFERENCES quizzes(qz_id) ON DELETE CASCADE,
    qs_created_at      	TIMESTAMPTZ DEFAULT NOW()
);

-- SUBMISSION RESULTS
CREATE TABLE IF NOT EXISTS submission_results (
    sub_id              SERIAL PRIMARY KEY,
    sub_user_id         INT REFERENCES users(usr_id) ON DELETE CASCADE,
    sub_quiz_id         INT REFERENCES quizzes(qz_id) ON DELETE CASCADE,
    sub_score           FLOAT NOT NULL CHECK (sub_score >= 0),
    sub_is_passed       BOOLEAN NOT NULL,
    sub_submitted_at    TIMESTAMPTZ DEFAULT NOW()
);

-- ANSWERS
CREATE TABLE IF NOT EXISTS answers (
    ans_id              SERIAL PRIMARY KEY,
    ans_result_id       INT REFERENCES submission_results(sub_id) ON DELETE CASCADE,
    ans_question_id     INT REFERENCES questions(qs_id) ON DELETE CASCADE,
    ans_is_correct      BOOLEAN NOT NULL,
    ans_data            JSONB NOT NULL
);

-- QUIZ LIKES
CREATE TABLE IF NOT EXISTS quiz_likes (
    qzlk_user_id       	INT REFERENCES users(usr_id) ON DELETE CASCADE,
    qzlk_quiz_id     	  INT REFERENCES quizzes(qz_id) ON DELETE CASCADE,
	
	CONSTRAINT unique_quiz_like UNIQUE(qzlk_user_id, qzlk_quiz_id) DEFERRABLE INITIALLY IMMEDIATE
);

-- COMMENTS
CREATE TABLE IF NOT EXISTS comments (
    cmt_id              SERIAL PRIMARY KEY,
    cmt_user_id       	INT REFERENCES users(usr_id) ON DELETE CASCADE,
    cmt_quiz_id     	  INT REFERENCES quizzes(qz_id) ON DELETE CASCADE,
	  cmt_content			    TEXT NOT NULL,
	  cmt_created_at		  TIMESTAMPTZ DEFAULT NOW()
);

-- COMMENT LIKES
CREATE TABLE IF NOT EXISTS comment_likes (
    cmlk_user_id       INT REFERENCES users(usr_id) ON DELETE CASCADE,
    cmlk_comment_id    INT REFERENCES comments(cmt_id) ON DELETE CASCADE,
	
	CONSTRAINT unique_comment_like UNIQUE(cmlk_user_id, cmlk_comment_id) DEFERRABLE INITIALLY IMMEDIATE
);

-- USER FOLLOWERS
CREATE TABLE IF NOT EXISTS user_followers (
    ufl_follower_id     INT REFERENCES users(usr_id) ON DELETE CASCADE,
    ufl_followed_id     INT REFERENCES users(usr_id) ON DELETE CASCADE,

    CONSTRAINT unique_user_follow UNIQUE(ufl_follower_id, ufl_followed_id) DEFERRABLE INITIALLY IMMEDIATE
);

/*
data type for qs_key can be one of these: // key = dap an
// single-choice key:
OptionKeys {
  keys: [
    {
      id: number, // has to be unique for each key
      content: string,
      image_url?: string,
      is_correct: boolean, // only one key could be true, may check in sql layer or app layer
      explanation?: string
    },
    ...
  ]
}

// multiple-choice key:
OptionKeys {
  keys: [
    {
      content: string,
      image_url?: string,
      is_correct: boolean,
      explanation?: string
    },
    ...
  ]
}

// text-entry key:
TextKey {
  correct_entry: string,
  explanation?: string
}

data type for ans_data can be one of these:
// multiple-choice answer:
UserChoices {
  choices: [
    {
      option_index: number
    },
    ...
  ]
}

// single-choice answer:
UserChoice {
    option_index: number
}

// text-entry answer:
UserEntry {
  entry: string
}
*/
