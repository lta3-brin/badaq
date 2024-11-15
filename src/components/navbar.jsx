import Plotly from "plotly.js-dist-min";
import { X, Save, ChevronDown } from "lucide-solid";
import { createEffect, useContext } from "solid-js";

import { AppContext } from "../stores";

export default function DefaultNavbar() {
  const { state, setState } = useContext(AppContext);

  createEffect(() => {
    if (state.k1.data) {
      let layout;

      if (state.isDark) {
        layout = {
          paper_bgcolor: "#2a303c",
          plot_bgcolor: "#2a303c",
          "xaxis.color": "#b2ccd6",
          "yaxis.color": "#b2ccd6",
          "legend.font.color": "#b2ccd6",
        };
      } else {
        layout = {
          paper_bgcolor: "#d8dee9",
          plot_bgcolor: "#d8dee9",
          "xaxis.color": "#444",
          "yaxis.color": "#444",
          "legend.font.color": "#444",
        };
      }

      Plotly.relayout(state.k1, layout);
      Plotly.relayout(state.k2, layout);
      Plotly.relayout(state.k3, layout);
      Plotly.relayout(state.k4, layout);
      Plotly.relayout(state.k5, layout);
      Plotly.relayout(state.k6, layout);
    }
  });

  return (
    <div class="navbar bg-primary text-primary-content shadow-sm">
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
              class="border border-secondary dropdown-content bg-primary rounded-box z-10 p-2 shadow-2xl space-y-1"
            >
              <li>
                <input
                  type="radio"
                  name="theme-dropdown"
                  class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                  aria-label="Light"
                  value="default"
                  onChange={() => setState("isDark", !state.isDark)}
                />
              </li>
              <li>
                <input
                  type="radio"
                  name="theme-dropdown"
                  class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                  aria-label="Dark"
                  value="business"
                  onChange={() => setState("isDark", !state.isDark)}
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
