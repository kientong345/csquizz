export interface Category {
  id: number;
  name: string;
  imageUrl?: string;
  description?: string;
}

export interface CreateCategoryRequest {
  name: string;
  imageUrl?: string;
  description?: string;
}

export interface UpdateCategoryRequest {
  name?: string;
  imageUrl?: string;
  description?: string;
}
