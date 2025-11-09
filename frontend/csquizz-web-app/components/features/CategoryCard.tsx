import Link from 'next/link';
import Image from 'next/image';
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { slugify } from '@/lib/utils';

type CategoryCardProps = {
  id: number;
  name: string;
  imageUrl?: string;
  description?: string;
};

export default function CategoryCard({
  id,
  name,
  imageUrl,
  description,
}: CategoryCardProps) {
  const quizzesHref = {
    pathname: `/${slugify(name)}/quizzes`,
    query: { categoryId: id },
  };

  return (
    <Card className="flex flex-col h-full hover:border-primary transition-colors overflow-hidden">
      {imageUrl && (
        <div className="relative w-full h-40 md:h-48 lg:h-56 bg-muted/40 overflow-hidden">
          <Image
            src={imageUrl}
            alt={name}
            fill
            sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw"
            className="object-cover"
          />
        </div>
      )}
      <CardHeader>
        <CardTitle>{name}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <div className="flex-grow" />
      <CardFooter>
        <Link href={quizzesHref} className="w-full">
          <Button className="w-full">Bắt đầu</Button>
        </Link>
      </CardFooter>
    </Card>
  );
}
