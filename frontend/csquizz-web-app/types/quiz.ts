export enum QuizDifficulty {
  Easy,
  Medium,
  Hard,
}

export interface QuizMetadata {
  id: number;
  title: string;
  description?: string;
  category: string;
  question_count: number;
  difficulty?: QuizDifficulty;
  created_by?: string;
}

export interface QuizQuery {
  category_id?: number;
  title_pattern?: string;
  difficulty?: string;
  created_by?: number;
  completed_by?: number;
  page: number;
  size: number;
}

export interface PostQuizMetadata{
  title: string;
  description?: string;
  category_id: number;
  difficulty?: QuizDifficulty;
  creator_id?: number;
}
