import Link from 'next/link';
import { Button } from '@/components/ui/button';

type QuizInfoCardProps = {
  id: number;
  title: string;
  difficulty: string;
  questionCount: number;
};

export default function QuizInfoCard({
  id,
  title,
  difficulty,
  questionCount,
}: QuizInfoCardProps) {
  return (
    <div className="bg-card text-card-foreground border rounded-lg p-4 flex items-center justify-between hover:border-primary transition-colors">
      <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-6">
        <h3 className="text-lg font-semibold mb-2 sm:mb-0">{title}</h3>
        <div className="flex items-center space-x-4 text-sm text-muted-foreground">
          <span>Độ khó: {difficulty}</span>
          <span>Số câu: {questionCount}</span>
        </div>
      </div>
      <Link href={`/quiz/${id}`}>
        <Button>Làm bài</Button>
      </Link>
    </div>
  );
}
