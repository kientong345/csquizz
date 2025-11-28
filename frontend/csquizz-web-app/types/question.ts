export type QuestionType = 'multiple_choice' | 'single_choice' | 'text_entry';

// Question Key Structures (snake_case as per API specs)
export interface QuestionKeyOption {
  id: number;
  content: string;
  image_url?: string;
  is_correct: boolean;
  explanation?: string;
}

export interface QuestionKey {
  keys?: QuestionKeyOption[];
  correct_entry?: string;
  explanation?: string;
}

// Question Data Structures
export interface Question {
  id: number;
  type: QuestionType;
  content: string;
  imageUrl?: string;
  publicData?: any; // This might need refinement based on what publicData actually returns for users
  quizId: number;
  createdAt: string;
}

export interface QuestionCreateParams {
  type: QuestionType;
  content: string;
  imageUrl?: string;
  key: QuestionKey;
}

export interface QuestionUpdateParams {
  type?: QuestionType;
  content?: string;
  imageUrl?: string;
  key?: QuestionKey;
}

// Answer Data Structures (snake_case as per API specs)
export interface AnswerDataChoice {
  option_id: number;
}

export interface AnswerData {
  choices?: AnswerDataChoice[];
  entry?: string;
}

export interface AnswerParam {
  questionId: number;
  data: AnswerData;
}
