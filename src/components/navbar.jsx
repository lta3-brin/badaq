import { X, Save, ChevronDown } from "lucide-solid";
import { createEffect, createSignal } from "solid-js";

export default function DefaultNavbar() {
  const [isdark, setIsdark] = createSignal(
    JSON.parse(localStorage.getItem("isdark"))
  );

  createEffect(() => {
    localStorage.setItem("isdark", JSON.stringify(isdark()));
  });

  return (
    <div class="navbar bg-primary text-primary-content shadow-sm sticky">
      <div class="flex-1">
        <span class="pl-2 text-xl">Dashboard</span>
      </div>
      <div class="flex-none space-x-2">
        <div
          class="tooltip tooltip-left tooltip-secondary"
          data-tip="Clear plots"
        >
          <button class="btn btn-square btn-ghost hover:text-secondary">
            <X size={28} />
          </button>
        </div>

        <div
          class="tooltip tooltip-left tooltip-secondary"
          data-tip="Change theme"
        >
          <div class="dropdown dropdown-end">
            <div
              tabindex="0"
              role="button"
              class="btn btn-square btn-ghost hover:text-secondary m-1"
            >
              <ChevronDown size={28} />
            </div>
            <ul
              tabindex="0"
              class="border border-secondary dropdown-content bg-primary rounded-box z-[1] p-2 shadow-2xl space-y-1"
            >
              <li>
                <input
                  type="radio"
                  name="theme-dropdown"
                  class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                  aria-label="Light"
                  value="default"
                  onChange={() => setIsdark(!isdark())}
                />
              </li>
              <li>
                <input
                  type="radio"
                  name="theme-dropdown"
                  class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                  aria-label="Dark"
                  value="business"
                  onChange={() => setIsdark(!isdark())}
                />
              </li>
            </ul>
          </div>
        </div>

        <div
          class="tooltip tooltip-left tooltip-secondary"
          data-tip="Save data"
        >
          <button class="btn btn-square btn-ghost hover:text-secondary">
            <Save size={28} />
          </button>
        </div>
      </div>
    </div>
  );
}
