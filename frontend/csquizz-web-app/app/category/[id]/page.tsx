
import { Input } from "@/components/ui/input";
import QuizInfoCard from "@/components/features/QuizInfoCard";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import { unslugify } from "@/lib/utils";

type QuizListPageProps = {
  params: {
    slug: string;
  };
};

const mockQuizzes = [
  { id: 101, title: "Basic Data Structures", difficulty: "Dễ", questionCount: 10 },
  { id: 102, title: "Trees and Graphs", difficulty: "Trung bình", questionCount: 15 },
  { id: 103, title: "Advanced Hashing", difficulty: "Khó", questionCount: 20 },
  { id: 104, title: "Linked List Manipulations", difficulty: "Trung bình", questionCount: 12 },
  // Add more mock quizzes to make pagination meaningful
  { id: 105, title: "Array Fundamentals", difficulty: "Dễ", questionCount: 10 },
  { id: 106, title: "Sorting Algorithms", difficulty: "Trung bình", questionCount: 15 },
  { id: 107, title: "Dynamic Programming Basics", difficulty: "Khó", questionCount: 20 },
  { id: 108, title: "Bit Manipulation", difficulty: "Khó", questionCount: 18 },
  { id: 109, title: "Recursion Techniques", difficulty: "Trung bình", questionCount: 12 },
  { id: 110, title: "Object-Oriented Design Patterns", difficulty: "Khó", questionCount: 25 },
];

export default function QuizListPage({ params }: QuizListPageProps) {
  const categoryName = unslugify(params.slug);

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
          {mockQuizzes.map((quiz) => (
            <QuizInfoCard
              key={quiz.id}
              id={quiz.id}
              title={quiz.title}
              difficulty={quiz.difficulty}
              questionCount={quiz.questionCount}
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
