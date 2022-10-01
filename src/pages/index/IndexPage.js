import { defineComponent, onMounted, ref } from 'vue'
import Chart from 'chart.js/auto'
import { Line } from 'vue-chartjs'

import { useForceStore } from 'src/stores/force'

export default defineComponent({
  name: 'IndexPage',
  components: {
    Line
  },
  setup() {
    const store = useForceStore()

    const chartOptionsFx = {
      responsive: true,
      scales: {
        x: {
          title: {
            display: true,
            text: 'DSN'
          }
        },
        y: {
          title: {
            display: true,
            text: 'Fx (N)'
          }
        }
      }
    }

    const chartOptionsFy = {
      responsive: true,
      scales: {
        x: {
          title: {
            display: true,
            text: 'DSN'
          }
        },
        y: {
          title: {
            display: true,
            text: 'Fy (N)'
          }
        }
      }
    }

    const chartOptionsFz = {
      responsive: true,
      scales: {
        x: {
          title: {
            display: true,
            text: 'DSN'
          }
        },
        y: {
          title: {
            display: true,
            text: 'Fz (N)'
          }
        }
      }
    }

    const chartOptionsMx = {
      responsive: true,
      scales: {
        x: {
          title: {
            display: true,
            text: 'DSN'
          }
        },
        y: {
          title: {
            display: true,
            text: 'Mx (Nm)'
          }
        }
      }
    }

    const chartOptionsMy = {
      responsive: true,
      scales: {
        x: {
          title: {
            display: true,
            text: 'DSN'
          }
        },
        y: {
          title: {
            display: true,
            text: 'My (Nm)'
          }
        }
      }
    }

    const chartOptionsMz = {
      responsive: true,
      scales: {
        x: {
          title: {
            display: true,
            text: 'DSN'
          }
        },
        y: {
          title: {
            display: true,
            text: 'Mz (Nm)'
          }
        }
      }
    }

    const initChartData = () => {
      return {
        labels: [],
        datasets: [
          {
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          }
        ]
      }
    }

    const chartData1 = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ${store.sec}`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    const chartData2 = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ${store.sec}`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    const chartData3 = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ${store.sec}`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    const chartData4 = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ${store.sec}`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    const chartData5 = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ${store.sec}`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    const chartData6 = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ${store.sec}`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    const label_length = ref(0)

    onMounted(() => {
      let socket = new WebSocket("ws://localhost:9001")

      socket.onmessage = function (e) {
        const data = e.data.split(",")

        if (data[0] === 'ENDSEQ') {
          label_length.value = store.lbl.length

          store.p1 = []
          store.p2 = []
          store.p3 = []
          store.p4 = []
          store.p5 = []
          store.p6 = []

          console.log(chartData1.value)
          chartData1.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          })

          chartData2.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          })

          chartData3.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          })

          chartData4.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          })

          chartData5.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          })

          chartData6.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: 'rgb(233, 69, 96)'
          })
        } else if (data[0] === 'connected') {
          store.connected = true
        } else if (data[0] === 'EXP') {
          console.log(data[0])
        } else if (data[0] !== 'ENDRUN') {
          store.sec = parseInt(data[1]) + 1

          if (parseInt(data[3]) > label_length.value) {
            store.lbl.push(parseInt(data[3]))
          }

          store.p1.push(parseFloat(data[5]))
          store.p2.push(parseFloat(data[7]))
          store.p3.push(parseFloat(data[4]))
          store.p4.push(parseFloat(data[9]))
          store.p5.push(parseFloat(data[6]))
          store.p6.push(parseFloat(data[8]))

          // Fx
          chartData1.value.labels = store.lbl
          chartData1.value.datasets[parseInt(data[1]) - 1].data = store.p1

          // Fy
          chartData2.value.labels = store.lbl
          chartData2.value.datasets[parseInt(data[1]) - 1].data = store.p2

          // Fz
          chartData3.value.labels = store.lbl
          chartData3.value.datasets[parseInt(data[1]) - 1].data = store.p3

          // Mx
          chartData4.value.labels = store.lbl
          chartData4.value.datasets[parseInt(data[1]) - 1].data = store.p4

          // My
          chartData5.value.labels = store.lbl
          chartData5.value.datasets[parseInt(data[1]) - 1].data = store.p5

          // Mz
          chartData6.value.labels = store.lbl
          chartData6.value.datasets[parseInt(data[1]) - 1].data = store.p6
        }
      }
    })

    return {
      chartData1,
      chartData2,
      chartData3,
      chartData4,
      chartData5,
      chartData6,
      chartOptionsFx,
      chartOptionsFy,
      chartOptionsFz,
      chartOptionsMx,
      chartOptionsMy,
      chartOptionsMz,
    }
  }
})