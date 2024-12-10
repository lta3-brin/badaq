import {
  createResource,
  createSignal,
  onMount,
  Suspense,
  useContext,
} from "solid-js";
import { Channel, invoke } from "@tauri-apps/api/core";
import Plotly from "plotly.js-dist-min";

import DefaultCard from "./card";
import { AppContext } from "../stores";
import { SimpleDialog } from "./dialog";

export default function DefaultMonitor() {
  let refSimpleDialog;
  const [message, setMessage] = createSignal();
  const { state, setState } = useContext(AppContext);

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
    scrollZoom: false,
    displayModeBar: false,
    displaylogo: false,
  };

  const fetchTcp = async () => {
    const onEvent = new Channel();

    onEvent.onmessage = (msg) => {
      const dialog = document.getElementById("simple_dialog");

      setMessage(null);
      if (msg.includes("ERROR")) {
        setMessage(msg.replace("ERROR:", ""));

        dialog.showModal();
      } else if (msg.includes("LOST")) {
        setMessage("TCP connection lost");

        dialog.showModal();
      } else {
        dialog.close();

        if (msg.includes("SEQ")) {
          const payload = msg.split(",");

          if (payload.length === 1 || msg.includes("ENDSEQ")) {
            Plotly.addTraces(state.k1, {
              unik: `k1_${payload[0]}`,
              type: "scatter",
              mode: "lines",
              name: payload[0],
              x: [],
              y: [],
              error_y: {
                type: "data",
                array: [],
                visible: true,
              },
            });

            Plotly.addTraces(state.k2, {
              unik: `k2_${payload[0]}`,
              type: "scatter",
              mode: "lines",
              name: payload[0],
              x: [],
              y: [],
              error_y: {
                type: "data",
                array: [],
                visible: true,
              },
            });

            Plotly.addTraces(state.k3, {
              unik: `k3_${payload[0]}`,
              type: "scatter",
              mode: "lines",
              name: payload[0],
              x: [],
              y: [],
              error_y: {
                type: "data",
                array: [],
                visible: true,
              },
            });

            Plotly.addTraces(state.k4, {
              unik: `k4_${payload[0]}`,
              type: "scatter",
              mode: "lines",
              name: payload[0],
              x: [],
              y: [],
              error_y: {
                type: "data",
                array: [],
                visible: true,
              },
            });

            Plotly.addTraces(state.k5, {
              unik: `k5_${payload[0]}`,
              type: "scatter",
              mode: "lines",
              name: payload[0],
              x: [],
              y: [],
              error_y: {
                type: "data",
                array: [],
                visible: true,
              },
            });

            Plotly.addTraces(state.k6, {
              unik: `k6_${payload[0]}`,
              type: "scatter",
              mode: "lines",
              name: payload[0],
              x: [],
              y: [],
              error_y: {
                type: "data",
                array: [],
                visible: true,
              },
            });
          } else {
            Plotly.extendTraces(state.k1, {
              x: [[parseInt(payload[1].split("-")[1])]],
              y: [[parseFloat(payload[3])]],
              "error_y.array": [[parseFloat(payload[5])]],
            }, [
              state.k1.data.findIndex((el) => el.unik === `k1_${payload[0]}`),
            ]);

            Plotly.extendTraces(state.k2, {
              x: [[parseInt(payload[1].split("-")[1])]],
              y: [[parseFloat(payload[7])]],
              "error_y.array": [[parseFloat(payload[9])]],
            }, [
              state.k2.data.findIndex((el) => el.unik === `k2_${payload[0]}`),
            ]);

            Plotly.extendTraces(state.k3, {
              x: [[parseInt(payload[1].split("-")[1])]],
              y: [[parseFloat(payload[11])]],
              "error_y.array": [[parseFloat(payload[13])]],
            }, [
              state.k3.data.findIndex((el) => el.unik === `k3_${payload[0]}`),
            ]);

            Plotly.extendTraces(state.k4, {
              x: [[parseInt(payload[1].split("-")[1])]],
              y: [[parseFloat(payload[15])]],
              "error_y.array": [[parseFloat(payload[17])]],
            }, [
              state.k4.data.findIndex((el) => el.unik === `k4_${payload[0]}`),
            ]);

            Plotly.extendTraces(state.k5, {
              x: [[parseInt(payload[1].split("-")[1])]],
              y: [[parseFloat(payload[19])]],
              "error_y.array": [[parseFloat(payload[21])]],
            }, [
              state.k5.data.findIndex((el) => el.unik === `k5_${payload[0]}`),
            ]);

            Plotly.extendTraces(state.k6, {
              x: [[parseInt(payload[1].split("-")[1])]],
              y: [[parseFloat(payload[23])]],
              "error_y.array": [[parseFloat(payload[25])]],
            }, [
              state.k6.data.findIndex((el) => el.unik === `k6_${payload[0]}`),
            ]);
          }
        }
      }
    };

    await invoke("try_connect", {
      addr: import.meta.env.VITE_TCP_ADDR,
      onevent: onEvent,
    });
  };

  const [_, { refetch }] = createResource(fetchTcp);

  onMount(() => {
    refSimpleDialog.addEventListener("click", () => {
      refetch();
    });

    Plotly.newPlot(state.k1, [], layout, configs);
    Plotly.newPlot(state.k2, [], layout, configs);
    Plotly.newPlot(state.k3, [], layout, configs);
    Plotly.newPlot(state.k4, [], layout, configs);
    Plotly.newPlot(state.k5, [], layout, configs);
    Plotly.newPlot(state.k6, [], layout, configs);
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

      <Suspense fallback={<></>}>
        <SimpleDialog ref={refSimpleDialog} title="Service Unavailable">
          <p class="py-4">{message()}</p>
        </SimpleDialog>
      </Suspense>
    </div>
  );
}
