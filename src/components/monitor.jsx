import { createResource, createSignal, onMount, useContext } from "solid-js";
import { invoke, Channel } from "@tauri-apps/api/core";
import Plotly from "plotly.js-dist-min";

import DefaultCard from "./card";
import { AppContext } from "../stores";
import { SimpleDialog } from "./dialog";

export default function DefaultMonitor() {
  const onEvent = new Channel();
  const { state, setState } = useContext(AppContext);
  const [message, setMessage] = createSignal();

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

  onEvent.onmessage = (msg) => {
    if (msg.includes("ERROR")) {
      setMessage(msg.replace("ERROR:", ""));

      document.getElementById("simple_dialog").showModal();
    } else {
      console.log(message());
    }
  };

  const fetchTcp = async () => {
    await invoke("try_connect", {
      addr: import.meta.env.VITE_TCP_ADDR,
      onevent: onEvent,
    });
  };

  const [_] = createResource(fetchTcp);

  onMount(async () => {
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

    Plotly.newPlot(state.k1, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(state.k2, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(state.k3, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(state.k4, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(state.k5, [trace1_k1, trace2_k1], layout, configs);
    Plotly.newPlot(state.k6, [trace1_k1, trace2_k1], layout, configs);
  });

  return (
    <div class="container mx-auto px-4 py-4 space-y-8">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DefaultCard title="Force on x axis" description="X = DSN, Y = Fx (N)">
          <div ref={(el) => setState("k1", el)} class="w-full" />
        </DefaultCard>

        <DefaultCard title="Force on y axis" description="X = DSN, Y = Fy (N)">
          <div ref={(el) => setState("k2", el)} class="w-full" />
        </DefaultCard>

        <DefaultCard title="Force on z axis" description="X = DSN, Y = Fz (N)">
          <div ref={(el) => setState("k3", el)} class="w-full" />
        </DefaultCard>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <DefaultCard
          title="Moment on x axis"
          description="X = DSN, Y = Mx (Nm)"
        >
          <div ref={(el) => setState("k4", el)} class="w-full" />
        </DefaultCard>

        <DefaultCard
          title="Moment on y axis"
          description="X = DSN, Y = My (Nm)"
        >
          <div ref={(el) => setState("k5", el)} class="w-full" />
        </DefaultCard>

        <DefaultCard
          title="Moment on z axis"
          description="X = DSN, Y = Mz (Nm)"
        >
          <div ref={(el) => setState("k6", el)} class="w-full" />
        </DefaultCard>
      </div>

      <SimpleDialog title="Service Unavailable" description={message()} />
    </div>
  );
}
