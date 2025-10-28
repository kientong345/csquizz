import Link from 'next/link';
import Image from 'next/image';

export default function Navbar() {
  return (
    <header className="bg-background border-b">
      <nav className="container mx-auto px-4 sm:px-6 lg:px-8 flex items-center justify-between h-16">
        <div className="flex items-center">
          <Link href="/" className="flex items-center space-x-2">
            <Image
              src="/csquizz-logo.svg"
              alt="CSQuizz Logo"
              width={120}
              height={30}
            />
          </Link>
        </div>
        <div className="flex items-center space-x-4">
          <Link
            href="/login"
            className="text-sm font-medium text-muted-foreground hover:text-foreground"
          >
            Đăng nhập
          </Link>
          <Link
            href="/register"
            className="text-sm font-medium text-white bg-primary hover:bg-primary/90 px-4 py-2 rounded-md"
          >
            Đăng ký
          </Link>
        </div>
      </nav>
    </header>
  );
}
