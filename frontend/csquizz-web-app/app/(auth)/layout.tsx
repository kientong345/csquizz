import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Auth - CSQuizz',
  description: 'Login or register for CSQuizz',
};

export default function AuthLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return <div className="min-h-screen flex items-center justify-center">{children}</div>;
}
