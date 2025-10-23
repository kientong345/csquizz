-- Add down migration script here
DROP INDEX idx_answers_selected_option;

DROP INDEX idx_answers_question_id;

DROP INDEX idx_answers_result_id;

DROP INDEX idx_results_quiz_id;

DROP INDEX idx_results_user_id;

DROP INDEX idx_options_question_id;

DROP INDEX idx_questions_quiz_id;

DROP INDEX idx_quizzes_created_by;

DROP INDEX idx_quizzes_category;
