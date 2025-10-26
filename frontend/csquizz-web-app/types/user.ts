export enum UserRole {
  User,
  Admin,
}

export interface UserPubInfo {
  id: number;
  displayName: string;
  avatarUrl?: string;
  role: UserRole;
  quizCreatedCount: number;
  quizCompletedCount: number;
}

export interface UserPubInfoPage {
  items: UserPubInfo[];
  total_items: number;
  total_pages: number;
}

export interface UserFullDetail {
  pubInfo: UserPubInfo;
  email: string;
  passwordHash?: string;
  googleId?: string;
}
