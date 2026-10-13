export default function Card({ children }: { children: React.ReactNode }) {
  return (
    <section className="bg-surface dark:bg-surface-dark p-6 rounded-lg shadow-md">
      {children}
    </section>
  );
}
