'use client';

import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination';
import { PAGE_WINDOW_SIZE } from '@/constants';
import { usePathname, useSearchParams } from 'next/navigation';

const generatePagination = (currentPage: number, totalPages: number) => {
  const pages: number[] = [];

  if (totalPages <= PAGE_WINDOW_SIZE) {
    for (let i = 1; i <= totalPages; i++) {
      pages.push(i);
    }
    return pages;
  }

  let startPage = Math.max(1, currentPage - Math.floor(PAGE_WINDOW_SIZE / 2));
  let endPage = Math.min(totalPages, startPage + PAGE_WINDOW_SIZE - 1);

  // Adjust window if it hits boundaries
  if (endPage - startPage + 1 < PAGE_WINDOW_SIZE) {
    if (startPage === 1) {
      endPage = PAGE_WINDOW_SIZE;
    } else if (endPage === totalPages) {
      startPage = totalPages - PAGE_WINDOW_SIZE + 1;
    }
  }

  // Ensure startPage is at least 1 and endPage is at most totalPages
  startPage = Math.max(1, startPage);
  endPage = Math.min(totalPages, endPage);

  for (let i = startPage; i <= endPage; i++) {
    pages.push(i);
  }

  return pages;
};

export type CSQuizzPaginationProps = {
  totalPages: number;
  className?: string;
};

const CSQuizzPagination = ({
  totalPages,
  className,
}: CSQuizzPaginationProps) => {
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const currentPage = Number(searchParams.get('page')) || 1;

  const createPageURL = (pageNumber: number | string) => {
    const params = new URLSearchParams(searchParams);
    params.set('page', pageNumber.toString());
    return `${pathname}?${params.toString()}`;
  };

  const allPages = generatePagination(currentPage, totalPages);

  return (
    <Pagination className={className}>
      <PaginationContent>
        <PaginationItem>
          <PaginationPrevious
            href={createPageURL(currentPage - 1)}
            className={currentPage <= 1 ? 'pointer-events-none opacity-50' : ''}
          />
        </PaginationItem>
        {allPages.map((page, index) => {
          return (
            <PaginationItem key={index}>
              <PaginationLink
                href={createPageURL(page.toString())}
                isActive={page === currentPage}
              >
                {page.toString()}
              </PaginationLink>
            </PaginationItem>
          );
        })}
        <PaginationItem>
          <PaginationNext
            href={createPageURL(currentPage + 1)}
            className={
              currentPage >= totalPages ? 'pointer-events-none opacity-50' : ''
            }
          />
        </PaginationItem>
      </PaginationContent>
    </Pagination>
  );
};

export default CSQuizzPagination;
