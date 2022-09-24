import { defineComponent, ref } from 'vue'

export default defineComponent({
  name: 'MainLayout',
  setup () {
    return {
      showDialog: ref(false)
    }
  }
})
