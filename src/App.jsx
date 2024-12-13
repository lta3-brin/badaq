import { lazy } from "solid-js";

import "./styles/index.css";

export default function App() {
  const DefaultNavrbar = lazy(() => import("./components/navbar"));
  const DefaultMonitor = lazy(() => import("./components/monitor"));

  return (
    <main class="">
      <DefaultNavrbar />

      <DefaultMonitor />
    </main>
  );
}
