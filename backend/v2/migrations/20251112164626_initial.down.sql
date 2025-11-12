-- Drop tables in reverse order of creation to avoid foreign key constraints
DROP TABLE IF EXISTS comment_likes;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS quiz_likes;
DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS submission_results;
DROP TABLE IF EXISTS questions;
DROP TABLE IF EXISTS quizzes;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS users;

-- Drop ENUM types after all tables that use them are dropped
DROP TYPE IF EXISTS question_type;
DROP TYPE IF EXISTS quiz_difficulty;
DROP TYPE IF EXISTS user_role;
