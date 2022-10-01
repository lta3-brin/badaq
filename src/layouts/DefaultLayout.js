import { defineComponent, ref, computed } from 'vue'
import { useForceStore } from 'src/stores/force'

export default defineComponent({
  name: 'MainLayout',
  setup () {
    const store = useForceStore()
    const showDialog = ref(false)
    const isConnected = computed(() => {
      return store.connected
    })

    return {
      showDialog,
      isConnected
    }
  }
})
