import { Input } from "@/components/ui/input";
import QuizCategory from "@/components/features/QuizCategory";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

const mockCategories = [
  {
    id: 1,
    name: "Data Structures",
    description: "Test your knowledge on arrays, linked lists, trees, and more.",
    imageUrl: "/category-data-structures.svg",
  },
  {
    id: 2,
    name: "Algorithms",
    description: "Challenge yourself with sorting, searching, and graph algorithms.",
    imageUrl: "/category-algorithms.svg",
  },
  {
    id: 3,
    name: "Operating Systems",
    description: "Dive into concepts like processes, memory management, and concurrency.",
    imageUrl: "/category-operating-systems.svg",
  },
  {
    id: 4,
    name: "Networking",
    description: "Explore the fundamentals of network protocols and layers.",
    imageUrl: "/category-networking.svg",
  },
  {
    id: 5,
    name: "Databases",
    description: "Understand SQL, normalization, and database design principles.",
    imageUrl: "/category-databases.svg",
  },
  {
    id: 6,
    name: "Artificial Intelligence",
    description: "Get started with the basic concepts of AI and machine learning.",
    imageUrl: "/category-ai.svg",
  },
];

export default function HomePage() {
  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <section className="text-center my-8 md:my-12">
        <h1 className="text-3xl md:text-5xl font-bold tracking-tight mb-4">
          Luyện tập kiến thức Khoa học máy tính
        </h1>
        <p className="text-lg md:text-xl text-muted-foreground mx-auto max-w-3xl">
          Chọn một chủ đề bên dưới để bắt đầu bài kiểm tra và thử thách kiến thức của bạn.
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
          {mockCategories.map((category) => (
            <QuizCategory
              key={category.id}
              id={category.id}
              name={category.name}
              description={category.description}
              imageUrl={category.imageUrl}
            />
          ))}
        </div>
      </section>

      <section className="mt-12 flex justify-center">
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious href="#" />
            </PaginationItem>
            <PaginationItem>
              <PaginationLink href="#">1</PaginationLink>
            </PaginationItem>
            <PaginationItem>
              <PaginationLink href="#" isActive>
                2
              </PaginationLink>
            </PaginationItem>
            <PaginationItem>
              <PaginationLink href="#">3</PaginationLink>
            </PaginationItem>
            <PaginationItem>
              <PaginationEllipsis />
            </PaginationItem>
            <PaginationItem>
              <PaginationNext href="#" />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      </section>
    </div>
  );
}