-- Add up migration script here
CREATE INDEX idx_quizzes_category ON quizzes (category);

CREATE INDEX idx_quizzes_created_by ON quizzes (created_by);

CREATE INDEX idx_questions_quiz_id ON questions (quiz_id);

CREATE INDEX idx_options_question_id ON options (question_id);

CREATE INDEX idx_results_user_id ON results (user_id);

CREATE INDEX idx_results_quiz_id ON results (quiz_id);

CREATE INDEX idx_answers_result_id ON user_answers (result_id);

CREATE INDEX idx_answers_question_id ON user_answers (question_id);

CREATE INDEX idx_answers_selected_option ON user_answers (selected_option);
