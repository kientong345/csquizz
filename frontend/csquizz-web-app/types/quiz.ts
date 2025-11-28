import { PaginatedResponse } from './common';
import { Question, QuestionCreateParams, AnswerParam } from './question';

export interface Quiz {
  id: number;
  title: string;
  description?: string;
  difficulty: string;
  categoryId: number;
  creatorId: number;
  passScore: number;
  createdAt: string;
  updatedAt: string;
  questionCount: number;
  likeCount: number;
  categoryName: string;
}

export interface QuizDetail extends Quiz {
  commentCount: number;
}

export interface QuizWithQuestions {
  quiz: QuizDetail;
  questions: PaginatedResponse<Question>;
}

export interface QuizParams {
  title: string;
  description?: string;
  difficulty?: string;
  categoryId: number;
  passScore: number;
}

export interface CreateQuizRequest {
  quizParams: QuizParams;
  questionsParams: QuestionCreateParams[];
}

export interface UpdateQuizRequest {
  title?: string;
  description?: string;
  difficulty?: string;
  categoryId?: number;
  passScore?: number;
}

export interface SubmitQuizRequest {
  answersParams: AnswerParam[];
}

export interface QuizQuery {
  page?: number;
  pageSize?: number;
  titlePattern?: string;
  categoryId?: number;
  difficulty?: string;
}
