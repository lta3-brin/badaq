import { defineComponent, computed} from "vue"
import Chart from 'chart.js/auto'
import { Line } from 'vue-chartjs'

import { useForceStore } from "src/stores/force"

export default defineComponent({
  name: 'DaPlotComponent',
  components: {
    Line
  },
  props: {
    title: {type: String, required: true},
    chartId: {
      type: String,
      default: 'k1'
    },
  },
  setup(props) {
    const store = useForceStore()
    const chartOptions = {
      responsive: true
    }

    const chartData = computed(() => {
      const obj = store[`${props.chartId}`]
      const datasets = []

      for (const key in obj) {
        const dt = obj[key]

        datasets.push({
          label: key,
          data: [],
          backgroundColor: 'rgb(83, 52, 131)',
          borderColor: 'rgb(233, 69, 96)'
        })
      }

      return {
        labels: [...Array(obj.SEQ01.length).keys()],
        datasets
      }
    })

    return {
      chartData,
      chartOptions
    }
  }
})
