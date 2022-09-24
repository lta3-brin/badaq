import { defineComponent , onMounted} from "vue"
import {Chart, registerables} from "chart.js"

import { useForceStore } from "src/stores/force"

export default defineComponent({
  name: 'DaPlotComponent',
  props: {
    id: {type: Number, required: true},
    title: {type: String, required: true},
  },
  setup(props) {
    const store = useForceStore()

    onMounted(() => {
      Chart.register(...registerables)

      const datasets = []

      for (const key in store[`k${props.id}`]) {
        datasets.push({
          label: key,
          backgroundColor: 'black',
          borderColor: `#${Math.floor(Math.random()*16777215).toString(16)}`,
          data: store[`k${props.id}`][key],
        })
      }

      const data = {
        labels: [...Array(store[`k${props.id}`]['SEQ1'].length).keys()],
        datasets
      }

      const config = {
        type: 'line',
        data: data,
        options: {
          layout: {
              padding: 0
          }
        }
      }

      const myChart = new Chart(
        document.getElementById(`k${props.id}`),
        config
      )
    })

    return {
      store
    }
  }
})
