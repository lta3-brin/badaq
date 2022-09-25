import { defineComponent, onMounted, computed, ref } from 'vue'
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
        } else if (data[0] !== 'ENDRUN') {
          store.sec = parseInt(data[1]) + 1
          store.lbl.push(parseInt(data[3]))
          store.p1.push(parseFloat(data[4]))
          store.p2.push(parseFloat(data[5]))
          store.p3.push(parseFloat(data[6]))
          store.p4.push(parseFloat(data[7]))
          store.p5.push(parseFloat(data[8]))
          store.p6.push(parseFloat(data[9]))

          chartData1.value.labels = store.lbl
          chartData1.value.datasets[parseInt(data[1]) - 1].data = store.p1

          chartData2.value.labels = store.lbl
          chartData2.value.datasets[parseInt(data[1]) - 1].data = store.p2

          chartData3.value.labels = store.lbl
          chartData3.value.datasets[parseInt(data[1]) - 1].data = store.p3

          chartData4.value.labels = store.lbl
          chartData4.value.datasets[parseInt(data[1]) - 1].data = store.p4

          chartData5.value.labels = store.lbl
          chartData5.value.datasets[parseInt(data[1]) - 1].data = store.p5

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
      chartOptions
    }
  }
})