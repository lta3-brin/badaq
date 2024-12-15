import Plotly from "plotly.js-dist-min";
import { save } from "@tauri-apps/plugin-dialog";
import { ChevronDown, Save, X } from "lucide-solid";
import { BaseDirectory, create } from "@tauri-apps/plugin-fs";
import { createEffect, createResource, useContext } from "solid-js";

import { AppContext } from "../stores";

export default function DefaultNavbar() {
  const { state } = useContext(AppContext);

  const fetchLocalStorage = async () => {
    const tema = await state.storage.get("temagelap");

    if (!tema) {
      state.storage.set("temagelap", false);
    }

    return tema;
  };

  const [temagelap, { mutate }] = createResource(fetchLocalStorage);

  createEffect(async () => {
    if (state.k1.data) {
      let layout;

      if (temagelap.latest) {
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

  const onBtnSave = async () => {
    const path = await save({
      filters: [
        {
          name: "Plot",
          extensions: [".csv", ".txt"],
        },
      ],
    });

    const file = await create(path, { baseDir: BaseDirectory.App });

    await file.write(new TextEncoder().encode("Hello world"));
    await file.close();
  };

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
                  checked={temagelap()}
                  onChange={() => {
                    const gelap = false;

                    mutate(gelap);
                    state.storage.set("temagelap", gelap);
                  }}
                />
              </li>
              <li>
                <input
                  type="radio"
                  name="theme-dropdown"
                  class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                  aria-label="Dark"
                  value="business"
                  checked={temagelap()}
                  onChange={() => {
                    const gelap = true;

                    mutate(gelap);
                    state.storage.set("temagelap", gelap);
                  }}
                />
              </li>
            </ul>
          </div>
        </div>

        <div
          class="tooltip tooltip-left tooltip-secondary"
          data-tip="Save data"
        >
          <button
            class="btn btn-square btn-ghost hover:text-secondary"
            onClick={onBtnSave}
          >
            <Save size={28} />
          </button>
        </div>
      </div>
    </div>
  );
}
