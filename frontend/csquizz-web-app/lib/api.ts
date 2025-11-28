import axios from 'axios';
import { Category, CategoryQuery } from '@/types/category';
import { Quiz, QuizQuery } from '@/types/quiz';
import { PaginatedResponse } from '@/types/common';

const apiClient = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

const defaultCategoryPage: PaginatedResponse<Category> = {
  items: [],
  totalItems: 0,
  totalPages: 0,
  currentPage: 1,
  pageSize: 10,
};

const defaultQuizPage: PaginatedResponse<Quiz> = {
  items: [],
  totalItems: 0,
  totalPages: 0,
  currentPage: 1,
  pageSize: 10,
};

/**
 * Fetches a paginated list of quiz categories from the backend.
 * @param {CategoryQuery} query - The query object with page and size.
 * @returns {Promise<PaginatedResponse<Category>>} A promise that resolves to a page of categories.
 */
export async function getCategories(
  query: CategoryQuery
): Promise<PaginatedResponse<Category>> {
  try {
    const url = `/categories?namePattern=${query.namePattern?query.namePattern:''}&page=${query.page}&pageSize=${query.pageSize}`;
    console.log(`url: ${url}`);
    const response = await apiClient.get(url);
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
 * @returns {Promise<PaginatedResponse<QuizMetadata>>} A promise that resolves to a page of quiz metadata.
 */
export async function getQuizzes(
  query: QuizQuery
): Promise<PaginatedResponse<Quiz>> {
  try {
    const url = `/quizzes?category_id=${query.categoryId}&page=${query.page}&pageSize=${query.pageSize}&sortBy=${query.sortBy}`;
    console.log(`url: ${url}`);
    const response = await apiClient.get(url);
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
