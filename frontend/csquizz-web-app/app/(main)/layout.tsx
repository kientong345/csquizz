// (main) group doesn't need a special layout, it inherits from root layout
// This file is optional but created for clarity and future customization
export default function MainLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return <>{children}</>;
}
