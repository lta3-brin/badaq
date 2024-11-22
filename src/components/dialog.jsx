export function SimpleDialog({ title, children, ref }) {
  return (
    <dialog id="simple_dialog" class="modal modal-bottom sm:modal-middle">
      <div class="modal-box">
        <h3 class="text-lg font-bold">{title}</h3>

        {children}

        <div class="modal-action">
          <form method="dialog">
            <button ref={ref} class="btn">
              Retry
            </button>
          </form>
        </div>
      </div>
    </dialog>
  );
}
