export function SimpleDialog({ title, description }) {
  return (
    <dialog id="simple_dialog" class="modal modal-bottom sm:modal-middle">
      <div class="modal-box">
        <h3 class="text-lg font-bold">{title}</h3>

        <p class="py-4">{description}</p>

        <div class="modal-action">
          <form method="dialog">
            <button class="btn">Close</button>
          </form>
        </div>
      </div>
    </dialog>
  );
}
