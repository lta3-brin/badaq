import { lazy } from "solid-js";
import "./styles/index.css";

export default function App() {
  const DefaultNavrbar = lazy(() => import("./components/navbar"));

  return (
    <main class="container">
      <DefaultNavrbar />
    </main>
  );
}
