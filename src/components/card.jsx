export default function DefaultCard({ title, description, children }) {
  return (
    <div class="card border border-base-300 bg-base-200 text-base-content shadow-xl w-full aspect-video">
      <div class="card-body">
        <h2 class="card-title">{title}</h2>
        <p>{description}</p>
      </div>
      <figure>{children}</figure>
    </div>
  );
}
