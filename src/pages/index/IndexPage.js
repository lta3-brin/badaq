import { defineComponent, onMounted } from 'vue'

import DaPlot from 'src/components/plot/DaPlot.vue'
import { useForceStore } from 'src/stores/force'

export default defineComponent({
  name: 'IndexPage',
  components: {
    DaPlot
  },
  setup() {
    const store = useForceStore()

    onMounted(() => {
      let socket = new WebSocket("ws://localhost:9001")

      socket.onmessage = function (e) {
        const data = e.data.split(",")

        if (data[0] === 'ENDSEQ') {
          store.p1 = []
          store.p2 = []
          store.p3 = []
          store.p4 = []
          store.p5 = []
          store.p6 = []
        } else if (data[0] !== 'ENDRUN') {
          store.$patch(state => {
            state.p1.push(parseFloat(data[2]))
            state.k1[`${data[0]}${data[1]}`] = state.p1

            state.p2.push(parseFloat(data[3]))
            state.k2[`${data[0]}${data[1]}`] = state.p2

            state.p3.push(parseFloat(data[4]))
            state.k3[`${data[0]}${data[1]}`] = state.p3

            state.p4.push(parseFloat(data[5]))
            state.k4[`${data[0]}${data[1]}`] = state.p4

            state.p5.push(parseFloat(data[6]))
            state.k5[`${data[0]}${data[1]}`] = state.p5

            state.p6.push(parseFloat(data[7]))
            state.k6[`${data[0]}${data[1]}`] = state.p6
          })
        }
      }
    })

    return {}
  }
})