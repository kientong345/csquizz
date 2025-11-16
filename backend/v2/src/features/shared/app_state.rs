use crate::features::{
    auth::service::AuthService, category::service::CategoryService,
    comment::service::CommentService, like::service::LikeService, quiz::service::QuizService,
    user::service::UserService,
};

pub struct AppState {
    pub auth_service: AuthService,
    pub category_service: CategoryService,
    pub comment_service: CommentService,
    pub like_service: LikeService,
    pub quiz_service: QuizService,
    pub user_service: UserService,
}
