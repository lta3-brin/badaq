import { defineComponent , onMounted, ref} from "vue"
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
    const isVisible = ref(false)

    onMounted(() => {
      Chart.register(...registerables)

      const datasets = []
      const obj = store[`k${props.id}`]

      if (Object.keys(obj).length === 0 && obj.constructor === Object) {
        isVisible.value = false
      } else {
        isVisible.value = true

        for (const key in obj) {
          datasets.push({
            label: key,
            backgroundColor: 'black',
            borderColor: `#${Math.floor(Math.random()*16777215).toString(16)}`,
            data: obj[key],
          })
        }
  
        const data = {
          labels: [...Array(Object.values(obj)[0].length).keys()],
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
  
        store.chart = new Chart(
          document.getElementById(`k${props.id}`),
          config
        )
      }
    })

    return {
      store,
      isVisible
    }
  }
})
