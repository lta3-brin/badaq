/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import { AppContext, setState, state } from "./stores";

render(
  () => (
    <AppContext.Provider value={{ state, setState }}>
      <App />
    </AppContext.Provider>
  ),
  document.getElementById("root")
);
