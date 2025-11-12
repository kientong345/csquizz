use std::sync::Arc;
use crate::{
    application::services::{
        auth_service::AuthService,
        category_service::CategoryService,
        comment_service::CommentService,
        like_service::LikeService,
        question_service::QuestionService,
        quiz_service::QuizService,
        submission_service::SubmissionService,
        user_service::UserService,
    },
    // This is a placeholder for the concrete implementation
    // that will live in the infrastructure layer.
    infrastructure::database::DbPool, 
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub category_service: CategoryService,
    pub comment_service: CommentService,
    pub like_service: LikeService,
    pub question_service: QuestionService,
    pub quiz_service: QuizService,
    pub submission_service: SubmissionService,
    pub user_service: UserService,
}

impl AppState {
    // This function will be called in main.rs to initialize the state.
    pub fn new(pool: Arc<DbPool>) -> Self {
        // Here we would initialize our concrete repositories
        // let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
        // let quiz_repo = Arc::new(PostgresQuizRepository::new(pool.clone()));
        // ... and so on for all repos

        // Then, we initialize services with the repository implementations
        // let auth_service = AuthService::new(user_repo.clone(), ...);
        // let user_service = UserService::new(user_repo.clone());
        // let quiz_service = QuizService::new(quiz_repo.clone());
        
        // For now, we'll use placeholders
        Self {
            auth_service: AuthService::default(),
            category_service: CategoryService::default(),
            comment_service: CommentService::default(),
            like_service: LikeService::default(),
            question_service: QuestionService::default(),
            quiz_service: QuizService::default(),
            submission_service: SubmissionService::default(),
            user_service: UserService::default(),
        }
    }
}
