import { Input } from '@/components/ui/input';
import CategoryCard from '@/components/features/CategoryCard';
import { getCategories } from '@/lib/api';
import { CATEGORY_PER_PAGE } from '@/constants';
import { Page } from '@/types/page';
import { Category } from '@/types/category';
import CSQuizzPagination from '@/components/features/CSQuizzPagination';

const mockCategoryPage: Page<Category> = {
  items: [
    {
      id: 1,
      name: 'Data Structures',
      description:
        'Test your knowledge on arrays, linked lists, trees, and more.',
      imageUrl: '/category-data-structures.svg',
    },
    {
      id: 2,
      name: 'Algorithms',
      description:
        'Challenge yourself with sorting, searching, and graph algorithms.',
      imageUrl: '/category-algorithms.svg',
    },
    {
      id: 3,
      name: 'Operating Systems',
      description:
        'Dive into concepts like processes, memory management, and concurrency.',
      imageUrl: '/category-operating-systems.svg',
    },
    {
      id: 4,
      name: 'Networking',
      description: 'Explore the fundamentals of network protocols and layers.',
      imageUrl: '/category-networking.svg',
    },
    {
      id: 5,
      name: 'Databases',
      description:
        'Understand SQL, normalization, and database design principles.',
      imageUrl: '/category-databases.svg',
    },
    {
      id: 6,
      name: 'Artificial Intelligence',
      description:
        'Get started with the basic concepts of AI and machine learning.',
      imageUrl: '/category-ai.svg',
    },
    {
      id: 7,
      name: 'Software Engineering',
      description:
        'Learn about software development methodologies and best practices.',
      imageUrl: '/category-software-engineering.svg',
    },
    {
      id: 8,
      name: 'Cybersecurity',
      description: 'Test your knowledge on security principles and practices.',
      imageUrl: '/category-cybersecurity.svg',
    },
    {
      id: 9,
      name: 'Web Development',
      description: 'Explore front-end and back-end web development concepts.',
      imageUrl: '/category-web-development.svg',
    },
    {
      id: 10,
      name: 'Programming Languages',
      description:
        'Understand different programming paradigms and language features.',
      imageUrl: '/category-programming-languages.svg',
    },
  ],
  total_items: 10,
  total_pages: 2,
};

function getPage(
  instance: Page<Category>,
  page: number,
  size: number
): Page<Category> {
  const start = (page - 1) * size;
  const end = start + size;
  return {
    items: instance.items.slice(start, end),
    total_items: instance.total_items,
    total_pages: Math.ceil(instance.total_items / size),
  };
}

export default async function HomePage({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) {
  const resolvedSearchParams = await searchParams;
  const page =
    typeof resolvedSearchParams.page === 'string'
      ? Number(resolvedSearchParams.page)
      : 1;

  let currentCategoryPage;
  if (process.env.NEXT_RUNTIME_ENV === 'production') {
    currentCategoryPage = await getCategories({ page: page, size: CATEGORY_PER_PAGE });
  } else {
    currentCategoryPage = getPage(
      mockCategoryPage,
      page,
      CATEGORY_PER_PAGE
    );
  }

  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <section className="text-center my-8 md:my-12">
        <h1 className="text-3xl md:text-5xl font-bold tracking-tight mb-4">
          Luyện tập kiến thức Khoa học máy tính
        </h1>
        <p className="text-lg md:text-xl text-muted-foreground mx-auto max-w-3xl">
          Chọn một chủ đề bên dưới để bắt đầu bài kiểm tra và thử thách kiến
          thức của bạn.
        </p>
      </section>

      <section className="mb-12">
        <div className="max-w-lg mx-auto">
          <Input
            type="search"
            placeholder="Tìm kiếm chủ đề... (ví dụ: Algorithms, Data Structures)"
            className="w-full text-base py-6"
          />
        </div>
      </section>

      <section>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {currentCategoryPage.items.map((category) => (
            <CategoryCard
              key={category.id}
              id={category.id}
              name={category.name}
              description={category.description}
              imageUrl={category.imageUrl}
            />
          ))}
        </div>
      </section>

      <div className="mt-8">
        <CSQuizzPagination totalPages={currentCategoryPage.total_pages} />
      </div>
    </div>
  );
}
