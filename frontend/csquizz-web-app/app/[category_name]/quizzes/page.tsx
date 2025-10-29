import { Input } from '@/components/ui/input';
import QuizInfoCard from '@/components/features/QuizInfoCard';
import { unslugify } from '@/lib/utils';
import { Page } from '@/types/page';
import { QuizDifficulty, QuizMetadata } from '@/types/quiz';
import CSQuizzPagination from '@/components/features/CSQuizzPagination';
import { getQuizzes } from '@/lib/api';
import { QUIZ_INFO_PER_PAGE } from '@/constants';

const mockQuizzes: Page<QuizMetadata> = {
  items: [
    {
      id: 101,
      title: 'Basic Data Structures',
      category: 'Data Structure',
      difficulty: QuizDifficulty.Easy,
      question_count: 10,
    },
    {
      id: 102,
      title: 'Trees and Graphs',
      category: 'Data Structure',
      difficulty: QuizDifficulty.Medium,
      question_count: 15,
    },
    {
      id: 103,
      title: 'Advanced Hashing',
      category: 'Algorithm',
      difficulty: QuizDifficulty.Hard,
      question_count: 20,
    },
    {
      id: 104,
      title: 'Linked List Manipulations',
      category: 'Algorithm',
      difficulty: QuizDifficulty.Medium,
      question_count: 12,
    },
    {
      id: 105,
      title: 'Array Fundamentals',
      category: 'Data Structure',
      difficulty: QuizDifficulty.Easy,
      question_count: 10,
    },
    {
      id: 106,
      title: 'Sorting Algorithms',
      category: 'Algorithm',
      difficulty: QuizDifficulty.Medium,
      question_count: 15,
    },
    {
      id: 107,
      title: 'Dynamic Programming Basics',
      category: 'Algorithm',
      difficulty: QuizDifficulty.Hard,
      question_count: 20,
    },
    {
      id: 108,
      title: 'Bit Manipulation',
      category: 'Algorithm',
      difficulty: QuizDifficulty.Hard,
      question_count: 18,
    },
    {
      id: 109,
      title: 'Recursion Techniques',
      category: 'Algorithm',
      difficulty: QuizDifficulty.Medium,
      question_count: 12,
    },
    {
      id: 110,
      title: 'Object-Oriented Design Patterns',
      category: 'OOP',
      difficulty: QuizDifficulty.Hard,
      question_count: 25,
    },
  ],
  total_items: 10,
  total_pages: 1,
};

function getPage(
  instance: Page<QuizMetadata>,
  page: number,
  size: number
): Page<QuizMetadata> {
  const start = (page - 1) * size;
  const end = start + size;
  return {
    items: instance.items.slice(start, end),
    total_items: instance.total_items,
    total_pages: Math.ceil(instance.total_items / size),
  };
}

function stringifyDifficulty(difficulty: QuizDifficulty | undefined): string {
  switch (difficulty) {
    case QuizDifficulty.Easy:
      return 'easy';
    case QuizDifficulty.Medium:
      return 'medium';
    case QuizDifficulty.Hard:
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
  const category_id =
    typeof resolvedSearchParams.category_id === 'string'
      ? Number(resolvedSearchParams.category_id)
      : -1;
  const page =
    typeof resolvedSearchParams.page === 'string'
      ? Number(resolvedSearchParams.page)
      : 1;

  const categoryName = unslugify(resolvedParams.category_name);
  const currentQuizPage = await getQuizzes({
    category_id: category_id,
    page: page,
    size: QUIZ_INFO_PER_PAGE,
  });
  // const currentQuizPage = getPage(mockQuizzes, page, QUIZ_INFO_PER_PAGE);

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
          {currentQuizPage.items.map((quiz) => (
            <QuizInfoCard
              key={quiz.id}
              id={quiz.id}
              title={quiz.title}
              difficulty={stringifyDifficulty(quiz.difficulty)}
              questionCount={quiz.question_count}
            />
          ))}
        </div>
      </section>

      <div className="mt-8">
        <CSQuizzPagination totalPages={currentQuizPage.total_pages} />
      </div>
    </div>
  );
}
