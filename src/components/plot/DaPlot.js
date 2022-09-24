import { defineComponent , onMounted} from "vue"
import {Chart, registerables} from "chart.js"

export default defineComponent({
  name: 'DaPlotComponent',
  props: {
    id: {type: Number, required: true},
    title: {type: String, required: true},
  },
  setup(props) {
    onMounted(() => {
      Chart.register(...registerables)

      const data = {
        labels: [...Array(7).keys()],
        datasets: [
          {
            label: 'SEQ1',
            backgroundColor: 'black',
            borderColor: `#${Math.floor(Math.random()*16777215).toString(16)}`,
            data: [0, 10, 5, 2, 20, 30, 45],
          },
          {
            label: 'SEQ2',
            backgroundColor: 'black',
            borderColor: `#${Math.floor(Math.random()*16777215).toString(16)}`,
            data: [0, 70, 25, 21, 80, 15, 5],
          }
        ]
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
      );
    })

    return {
    }
  }
})
