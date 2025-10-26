export enum QuestionForm {
  MultipleChoice,
  SingleChoice,
  TextEntry,
}

export interface AnswerOption {
  id: number;
  text: string;
}

export interface Question {
  id: number;
  form: QuestionForm;
  text: string;
  image_url?: string;
  explanation?: string;
  options: AnswerOption[],
}
