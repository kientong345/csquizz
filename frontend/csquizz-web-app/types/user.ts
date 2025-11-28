export interface User {
  id: number;
  displayName: string;
  avatarUrl?: string;
}

export interface UserDetail extends User {
  createdAt: string;
  quizCompletedCount: number;
  quizCreatedCount: number;
  followerCount: number;
}

export interface CurrentUser extends UserDetail {
  email: string;
  role: 'user' | 'admin';
}

export interface UpdateUserRequest {
  displayName?: string;
  passwordHash?: string;
  avatarUrl?: string;
}

export interface UserSubmission {
  id: number;
  score: number;
  isPassed: boolean;
  submittedAt: string;
  quizTitle: string;
}
