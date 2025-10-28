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
  return (
    <Card className="flex flex-col h-full hover:border-primary transition-colors overflow-hidden">
      {imageUrl && (
        <div className="relative w-full h-40 bg-muted/40">
          <Image
            src={imageUrl}
            alt={name}
            fill
            className="object-contain p-4"
          />
        </div>
      )}
      <CardHeader>
        <CardTitle>{name}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <div className="flex-grow" />
      <CardFooter>
        <Link href={`/category/${id}`} className="w-full">
          <Button className="w-full">Bắt đầu</Button>
        </Link>
      </CardFooter>
    </Card>
  );
}
