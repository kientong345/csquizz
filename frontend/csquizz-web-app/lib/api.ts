import axios from 'axios';
import { Category, CategoryQuery } from '@/types/category';
import { QuizMetadata, QuizQuery } from '@/types/quiz';
import { Page } from '@/types/page';

const apiClient = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

const defaultCategoryPage: Page<Category> = {
  items: [],
  total_items: 0,
  total_pages: 0,
};

const defaultQuizPage: Page<QuizMetadata> = {
  items: [],
  total_items: 0,
  total_pages: 0,
};

/**
 * Fetches a paginated list of quiz categories from the backend.
 * @param {CategoryQuery} query - The query object with page and size.
 * @returns {Promise<Page<Category>>} A promise that resolves to a page of categories.
 */
export async function getCategories(
  query: CategoryQuery
): Promise<Page<Category>> {
  try {
    const response = await apiClient.get('/categories', { params: query });
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      console.error('Axios error fetching categories:', error.message);
    } else {
      console.error('Unexpected error fetching categories:', error);
    }
    return defaultCategoryPage;
  }
}

/**
 * Fetches a paginated list of quizzes based on a flexible query object.
 * @param {QuizQuery} query - The query object with parameters like category_id, page, size, etc.
 * @returns {Promise<Page<QuizMetadata>>} A promise that resolves to a page of quiz metadata.
 */
export async function getQuizzes(
  query: QuizQuery
): Promise<Page<QuizMetadata>> {
  try {
    const response = await apiClient.get('/quizzes', { params: query });
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      console.error('Axios error fetching quizzes:', error.message);
    } else {
      console.error('Unexpected error fetching quizzes:', error);
    }
    return defaultQuizPage;
  }
}
