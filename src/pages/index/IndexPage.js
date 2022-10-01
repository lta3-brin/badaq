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

    const initChartOption = (lablex, labely) => {
      return {
        responsive: true,
        scales: {
          x: {
            title: {
              display: true,
              text: lablex
            }
          },
          y: {
            title: {
              display: true,
              text: labely
            }
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

    const chartOptionsFx = initChartOption('DSN', 'Fx (N)')
    const chartOptionsFy = initChartOption('DSN', 'Fy (N)')
    const chartOptionsFz = initChartOption('DSN', 'Fz (N)')
    const chartOptionsMx = initChartOption('DSN', 'Mx (Nm)')
    const chartOptionsMy = initChartOption('DSN', 'My (Nm)')
    const chartOptionsMz = initChartOption('DSN', 'Mz (Nm)')

    const chartData1 = ref(initChartData())
    const chartData2 = ref(initChartData())
    const chartData3 = ref(initChartData())
    const chartData4 = ref(initChartData())
    const chartData5 = ref(initChartData())
    const chartData6 = ref(initChartData())

    const label_length = ref(0)

    const randomRange = (min, max) => {
      min = Math.ceil(min)
      max = Math.floor(max)

      return Math.floor(Math.random() * (max - min + 1) + min)
    }

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

          const red = randomRange(0, 255)
          const green = randomRange(0, 255)
          const blue = randomRange(0, 255)
          chartData1.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: `rgb(${red}, ${green}, ${blue})`
          })

          chartData2.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: `rgb(${red}, ${green}, ${blue})`
          })

          chartData3.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: `rgb(${red}, ${green}, ${blue})`
          })

          chartData4.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: `rgb(${red}, ${green}, ${blue})`
          })

          chartData5.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: `rgb(${red}, ${green}, ${blue})`
          })

          chartData6.value.datasets.push({
            label: `SEQ${store.sec}`,
            data: [],
            backgroundColor: 'rgb(83, 52, 131)',
            borderColor: `rgb(${red}, ${green}, ${blue})`
          })
        } else if (data[0] === 'connected') {
          store.connected = true
        } else if (data[0] === 'EXP') {
          store.sec = 1
          chartData1.value = initChartData()
          chartData2.value = initChartData()
          chartData3.value = initChartData()
          chartData4.value = initChartData()
          chartData5.value = initChartData()
          chartData6.value = initChartData()
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
        } else if (data[0] === 'ENDRUN') {
          chartData1.value.datasets.pop()
          chartData2.value.datasets.pop()
          chartData3.value.datasets.pop()
          chartData4.value.datasets.pop()
          chartData5.value.datasets.pop()
          chartData6.value.datasets.pop()
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