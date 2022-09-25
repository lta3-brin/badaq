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

    const chartOptions = {
      responsive: true
    }

    const chartData = ref({
      labels: [],
      datasets: [
        {
          label: `SEQ01`,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        }
      ]
    })

    onMounted(() => {
      let socket = new WebSocket("ws://localhost:9001")

      socket.onmessage = function (e) {
        const data = e.data.split(",")

        if (data[0] === 'ENDSEQ') {
          store.lbl = []
          store.p1 = []
          store.p2 = []
          store.p3 = []
          store.p4 = []
          store.p5 = []
          store.p6 = []
        } else if (data[0] !== 'ENDRUN') {
          store.sec = parseInt(data[1])
          store.lbl.push(parseInt(data[3]))
          store.p1.push(parseFloat(data[4]))
          store.p2.push(parseFloat(data[5]))
          store.p3.push(parseFloat(data[6]))
          store.p4.push(parseFloat(data[7]))
          store.p5.push(parseFloat(data[8]))
          store.p6.push(parseFloat(data[9]))

          chartData.value.labels = store.lbl
          chartData.value.datasets[0].data = store.p1
        }
      }
    })

    return {
      chartData,
      chartOptions
    }
  }
})