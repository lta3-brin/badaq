import { lazy } from "solid-js";

export default function DefaultMonitor() {
  const DefaultCard = lazy(() => import("./card"));

  return (
    <div class="container mx-auto px-4 pt-4 space-y-8">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DefaultCard title="All Shoes!" description="">
          <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
          />
        </DefaultCard>

        <DefaultCard title="Basic usage" description="">
          <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
          />
        </DefaultCard>

        <DefaultCard title="Columns in a grid" description="">
          <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
          />
        </DefaultCard>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DefaultCard title="Responsive variants" description="">
          <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
          />
        </DefaultCard>

        <DefaultCard title="Centering by default" description="">
          <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
          />
        </DefaultCard>

        <DefaultCard title="Adding horizontal padding" description="">
          <img
            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
            alt="Shoes"
          />
        </DefaultCard>
      </div>
    </div>
  );
}
