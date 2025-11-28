import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Admin - CSQuizz',
  description: 'CSQuizz admin panel',
};

export default function AdminLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="min-h-screen bg-muted/20">
      <div className="border-b bg-background">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <h2 className="text-xl font-semibold">Admin Panel</h2>
        </div>
      </div>
      {children}
    </div>
  );
}
