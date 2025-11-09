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
  questionCount: number;
  difficulty?: QuizDifficulty;
  createdBy?: string;
}

export interface QuizQuery {
  categoryId?: number;
  titlePattern?: string;
  difficulty?: string;
  createdBy?: number;
  completedBy?: number;
  page: number;
  size: number;
}

export interface PostQuizMetadata{
  title: string;
  description?: string;
  categoryId: number;
  difficulty?: QuizDifficulty;
  creatorId?: number;
}
