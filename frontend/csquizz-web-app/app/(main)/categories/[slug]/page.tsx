import { Input } from '@/components/ui/input';
import QuizInfoCard from '@/components/features/QuizInfoCard';
import { unslugify } from '@/lib/utils';
import { PaginatedResponse } from '@/types/common';
import { Quiz } from '@/types/quiz';
import CSQuizzPagination from '@/components/features/CSQuizzPagination';
import { getQuizzes } from '@/lib/api';
import { QUIZ_INFO_PER_PAGE } from '@/constants';

const mockQuizzes: PaginatedResponse<Quiz> = {
  items: [
    {
      id: 101,
      title: 'Basic Data Structures',
      categoryId: 1,
      difficulty: 'easy',
      questionCount: 10,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
    },
    {
      id: 102,
      title: 'Trees and Graphs',
      categoryId: 2,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'medium',
      questionCount: 15,
    },
    {
      id: 103,
      title: 'Advanced Hashing',
      categoryId: 3,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'hard',
      questionCount: 20,
    },
    {
      id: 104,
      title: 'Linked List Manipulations',
      categoryId: 4,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'medium',
      questionCount: 12,
    },
    {
      id: 105,
      title: 'Array Fundamentals',
      categoryId: 5,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'easy',
      questionCount: 10,
    },
    {
      id: 106,
      title: 'Sorting Algorithms',
      categoryId: 6,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'medium',
      questionCount: 15,
    },
    {
      id: 107,
      title: 'Dynamic Programming Basics',
      categoryId: 7,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'hard',
      questionCount: 20,
    },
    {
      id: 108,
      title: 'Bit Manipulation',
      categoryId: 8,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'hard',
      questionCount: 18,
    },
    {
      id: 109,
      title: 'Recursion Techniques',
      categoryId: 9,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'medium',
      questionCount: 12,
    },
    {
      id: 110,
      title: 'Object-Oriented Design Patterns',
      categoryId: 10,
      creatorId: 1,
      passScore: 5,
      createdAt: '2023-01-01T00:00:00.000Z',
      updatedAt: '2023-01-01T00:00:00.000Z',
      likeCount: 0,
      categoryName: 'Data Structure',
      difficulty: 'hard',
      questionCount: 25,
    },
  ],
  totalItems: 10,
  totalPages: 1,
  currentPage: 1,
  pageSize: 10,
};

function getPage(
  instance: PaginatedResponse<Quiz>,
  page: number,
  size: number
): PaginatedResponse<Quiz> {
  const start = (page - 1) * size;
  const end = start + size;
  return {
    items: instance.items.slice(start, end),
    totalItems: instance.totalItems,
    totalPages: Math.ceil(instance.totalItems / size),
    currentPage: page,
    pageSize: size,
  };
}

function stringifyDifficulty(difficulty: string | undefined): string {
  switch (difficulty) {
    case 'easy':
      return 'easy';
    case 'medium':
      return 'medium';
    case 'hard':
      return 'hard';
    default:
      return 'none';
  }
}

export default async function QuizListPage({
  params,
  searchParams,
}: {
  params: { category_name: string };
  searchParams: { [key: string]: string | string[] | undefined };
}) {
  const resolvedSearchParams = await searchParams;
  const resolvedParams = await params;
  const categoryId =
    typeof resolvedSearchParams.categoryId === 'string'
      ? Number(resolvedSearchParams.categoryId)
      : -1;
  const page =
    typeof resolvedSearchParams.page === 'string'
      ? Number(resolvedSearchParams.page)
      : 1;

  const categoryName = unslugify(resolvedParams.category_name);

  let currentQuizPage;
  if (process.env.NEXT_RUNTIME_ENV === 'production') {
    currentQuizPage = await getQuizzes({
      categoryId: categoryId,
      page: page,
      pageSize: QUIZ_INFO_PER_PAGE,
    });
  } else {
    currentQuizPage = getPage(mockQuizzes, page, QUIZ_INFO_PER_PAGE);
  }

  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <section className="my-8 md:my-12">
        <h1 className="text-3xl md:text-5xl font-bold tracking-tight mb-4">
          Chủ đề: {categoryName}
        </h1>
        <div className="max-w-lg mt-6">
          <Input
            type="search"
            placeholder="Tìm kiếm quiz trong chủ đề này..."
            className="w-full text-base py-6"
          />
        </div>
      </section>

      <section>
        <div className="flex flex-col gap-4">
          {currentQuizPage.items.map((quiz: Quiz) => (
            <QuizInfoCard
              key={quiz.id}
              id={quiz.id}
              title={quiz.title}
              difficulty={stringifyDifficulty(quiz.difficulty)}
              questionCount={quiz.questionCount}
            />
          ))}
        </div>
      </section>

      <div className="mt-8">
        <CSQuizzPagination totalPages={currentQuizPage.totalPages} />
      </div>
    </div>
  );
}
