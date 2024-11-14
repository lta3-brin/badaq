import { onMount } from "solid-js";
import Plotly from "plotly.js-dist-min";

import DefaultCard from "./card";

export default function DefaultMonitor() {
  let k1;
  let k2;
  let k3;
  let k4;
  let k5;
  let k6;

  const layout = {
    responsive: true,
    showlegend: true,
    height: 300,
    paper_bgcolor: "#d8dee9",
    plot_bgcolor: "#d8dee9",
    legend: { orientation: "h" },
    margin: {
      l: 25,
      r: 15,
      b: 0,
      t: 0,
    },
    xaxis: {
      zeroline: false,
    },
    yaxis: {
      zeroline: false,
    },
  };

  const configs = {
    responsive: true,
    scrollZoom: true,
    displayModeBar: false,
    displaylogo: false,
  };

  onMount(() => {
    const trace1_k1 = {
      x: [0, 1, 2, 3, 4],
      y: [0, 10, 15, 13, 17],
      type: "scatter",
      name: "polar 1",
    };

    const trace2_k1 = {
      x: [1, 2, 3, 4],
      y: [16, 5, 11, 9],
      type: "scatter",
      name: "polar 2",
    };

    Plotly.newPlot(k1, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(k2, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(k3, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(k4, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(k5, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(k6, [trace1_k1, trace2_k1], layout, configs);
  });

  return (
    <div class="container mx-auto px-4 pt-4 space-y-8">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DefaultCard title="Force on x axis" description="X = DSN, Y = Fx (N)">
          <div ref={k1} class="w-full" />
        </DefaultCard>

        <DefaultCard title="Force on y axis" description="X = DSN, Y = Fy (N)">
          <div ref={k2} class="w-full" />
        </DefaultCard>

        <DefaultCard title="Force on z axis" description="X = DSN, Y = Fz (N)">
          <div ref={k3} class="w-full" />
        </DefaultCard>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DefaultCard
          title="Moment on x axis"
          description="X = DSN, Y = Mx (Nm)"
        >
          <div ref={k4} class="w-full" />
        </DefaultCard>

        <DefaultCard
          title="Moment on y axis"
          description="X = DSN, Y = My (Nm)"
        >
          <div ref={k5} class="w-full" />
        </DefaultCard>

        <DefaultCard
          title="Moment on z axis"
          description="X = DSN, Y = Mz (Nm)"
        >
          <div ref={k6} class="w-full" />
        </DefaultCard>
      </div>
    </div>
  );
}
