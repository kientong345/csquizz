export default function PublicUserPage({ params }: { params: { id: string } }) {
  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 className="text-3xl font-bold mb-4">User Profile - ID: {params.id}</h1>
      <p className="text-muted-foreground mb-8">
        This is a placeholder for viewing other users' public profiles.
      </p>
      {/* TODO: Implement public user profile view */}
    </div>
  );
}
