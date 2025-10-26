export interface Category {
  id: number;
  name: string;
  imageUrl?: string;
  description?: string;
}

export interface CategoryQuery {
	page: number;
	size: number;
}

export interface PostCategory {
	name: string;
	imageUrl?: string;
	description?: string;
}
