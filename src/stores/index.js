import { createContext } from "solid-js";
import { createStore } from "solid-js/store";
import { LazyStore } from '@tauri-apps/plugin-store';

export const AppContext = createContext()

export const [state, setState] = createStore({
    storage: new LazyStore("storage.json"),
    isDark: false,
    k1: null,
    k2: null,
    k3: null,
    k4: null,
    k5: null,
    k6: null
});