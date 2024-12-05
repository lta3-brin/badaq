import {
  createEffect,
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
import { createStore, produce } from "solid-js/store";

export default function DefaultMonitor() {
  let refSimpleDialog;
  const [message, setMessage] = createSignal();
  const { state, setState } = useContext(AppContext);
  const [store, setStore] = createStore({
    data1: [],
    data2: [],
    data3: [],
    data4: [],
    data5: [],
    data6: [],
  });

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

          if (payload.length === 1 && !msg.includes("ENDSEQ")) {
            setStore("data1", store.data1.length, {
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

            setStore("data2", store.data2.length, {
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

            setStore("data3", store.data3.length, {
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

            setStore("data4", store.data4.length, {
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

            setStore("data5", store.data5.length, {
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

            setStore("data6", store.data6.length, {
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
            setStore(
              "data1",
              (el) => el.unik === `k1_${payload[0]}`,
              produce((el) => {
                const x = parseInt(payload[1].split("-")[1]);
                const y = parseFloat(payload[3]);
                const y_std = parseFloat(payload[5]);

                el.x.push(x);
                el.y.push(y);
                el.error_y.array.push(y_std);
              }),
            );

            setStore(
              "data2",
              (el) => el.unik === `k2_${payload[0]}`,
              produce((el) => {
                const x = parseInt(payload[1].split("-")[1]);
                const y = parseFloat(payload[7]);
                const y_std = parseFloat(payload[9]);

                el.x.push(x);
                el.y.push(y);
                el.error_y.array.push(y_std);
              }),
            );

            setStore(
              "data3",
              (el) => el.unik === `k3_${payload[0]}`,
              produce((el) => {
                const x = parseInt(payload[1].split("-")[1]);
                const y = parseFloat(payload[11]);
                const y_std = parseFloat(payload[13]);

                el.x.push(x);
                el.y.push(y);
                el.error_y.array.push(y_std);
              }),
            );

            setStore(
              "data4",
              (el) => el.unik === `k4_${payload[0]}`,
              produce((el) => {
                const x = parseInt(payload[1].split("-")[1]);
                const y = parseFloat(payload[15]);
                const y_std = parseFloat(payload[17]);

                el.x.push(x);
                el.y.push(y);
                el.error_y.array.push(y_std);
              }),
            );

            setStore(
              "data5",
              (el) => el.unik === `k5_${payload[0]}`,
              produce((el) => {
                const x = parseInt(payload[1].split("-")[1]);
                const y = parseFloat(payload[19]);
                const y_std = parseFloat(payload[21]);

                el.x.push(x);
                el.y.push(y);
                el.error_y.array.push(y_std);
              }),
            );

            setStore(
              "data6",
              (el) => el.unik === `k6_${payload[0]}`,
              produce((el) => {
                const x = parseInt(payload[1].split("-")[1]);
                const y = parseFloat(payload[23]);
                const y_std = parseFloat(payload[25]);

                el.x.push(x);
                el.y.push(y);
                el.error_y.array.push(y_std);
              }),
            );
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
  });

  createEffect(async () => {
    Plotly.newPlot(state.k1, store.data1, layout, configs);
    Plotly.newPlot(state.k2, store.data2, layout, configs);
    Plotly.newPlot(state.k3, store.data3, layout, configs);
    Plotly.newPlot(state.k4, store.data4, layout, configs);
    Plotly.newPlot(state.k5, store.data5, layout, configs);
    Plotly.newPlot(state.k6, store.data6, layout, configs);
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
