import { Input } from '@/components/ui/input';
import CategoryCard from '@/components/features/CategoryCard';
import { getCategories } from '@/lib/api';
import { CATEGORY_PER_PAGE } from '@/constants';
import { PaginatedResponse } from '@/types/common';
import { Category } from '@/types/category';
import CSQuizzPagination from '@/components/features/CSQuizzPagination';

const mockCategoryPage: PaginatedResponse<Category> = {
  items: [
    {
      id: 1,
      name: 'Data Structures',
      description:
        'Test your knowledge on arrays, linked lists, trees, and more.',
      // imageUrl: '/category-data-structures.svg',
      imageUrl: 'https://images.shiksha.com/mediadata/ugcDocuments/images/wordpressImages/2020_05_2167870_21e8.jpg'
    },
    {
      id: 2,
      name: 'Algorithms',
      description:
        'Challenge yourself with sorting, searching, and graph algorithms.',
      // imageUrl: '/category-algorithms.svg',
      imageUrl: 'https://www.snexplores.org/wp-content/uploads/2020/11/1030_algorithm_explainer-1028x579.jpg'
    },
    {
      id: 3,
      name: 'Operating Systems',
      description:
        'Dive into concepts like processes, memory management, and concurrency.',
      // imageUrl: '/category-operating-systems.svg',
      imageUrl: 'https://cloudpso.com/wp-content/uploads/2024/01/ops2.jpg'
    },
    {
      id: 4,
      name: 'Networking',
      description: 'Explore the fundamentals of network protocols and layers.',
      // imageUrl: '/category-networking.svg',
      imageUrl: 'https://www.microsoft.com/en-us/research/wp-content/uploads/2018/08/01_MSR_SIGCOMM_Data_Network_1400x788.png'
    },
    {
      id: 5,
      name: 'Databases',
      description:
        'Understand SQL, normalization, and database design principles.',
      // imageUrl: '/category-databases.svg',
      imageUrl: 'https://techvccloud.mediacdn.vn/2020/11/4/database-la-gi-2-16044569615001962544461.png'
    },
    {
      id: 6,
      name: 'Artificial Intelligence',
      description:
        'Get started with the basic concepts of AI and machine learning.',
      // imageUrl: '/category-ai.svg',
      imageUrl: 'https://engineering.fb.com/wp-content/uploads/2019/05/grid-AI.jpg'
    },
    {
      id: 7,
      name: 'Software Engineering',
      description:
        'Learn about software development methodologies and best practices.',
      // imageUrl: '/category-software-engineering.svg',
      imageUrl: 'https://investin.org/cdn/shop/articles/software-engineering-skills_resize_md.jpg'
    },
    {
      id: 8,
      name: 'Cybersecurity',
      description: 'Test your knowledge on security principles and practices.',
      // imageUrl: '/category-cybersecurity.svg',
      imageUrl: 'https://www.iare.ac.in/sites/default/files/department_images/Cybersecurity.jpg'
    },
    {
      id: 9,
      name: 'Web Development',
      description: 'Explore front-end and back-end web development concepts.',
      // imageUrl: '/category-web-development.svg',
      imageUrl: 'https://spec.nith.ac.in/BLOGS/a1%20(5).jpg'
    },
    {
      id: 10,
      name: 'Programming Languages',
      description:
        'Understand different programming paradigms and language features.',
      // imageUrl: '/category-programming-languages.svg',
      imageUrl: 'https://binarapps.com/wp-content/uploads/2021/09/Top-10-Programming-Languages-of-the-Future.png'
    },
  ],
  totalItems: 10,
  totalPages: 2,
  currentPage: 1,
  pageSize: 10,
};

function getPage(
  instance: PaginatedResponse<Category>,
  page: number,
  size: number
): PaginatedResponse<Category> {
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
          {currentCategoryPage.items.map((category: Category) => (
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
        <CSQuizzPagination totalPages={currentCategoryPage.totalPages} />
      </div>
    </div>
  );
}
