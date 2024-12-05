import { createContext } from "solid-js";
import { createStore } from "solid-js/store";

export const AppContext = createContext()

export const [state, setState] = createStore({
    isDark: false,
    k1: null,
    k2: null,
    k3: null,
    k4: null,
    k5: null,
    k6: null
});