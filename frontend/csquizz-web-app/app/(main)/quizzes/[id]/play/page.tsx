export default function QuizPlayPage({ params }: { params: { id: string } }) {
  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 className="text-3xl font-bold mb-4">Quiz Play - ID: {params.id}</h1>
      <p className="text-muted-foreground mb-8">
        This is a placeholder for the quiz play page where users answer questions.
      </p>
      {/* TODO: Implement quiz questions, answer options, submit logic */}
    </div>
  );
}
