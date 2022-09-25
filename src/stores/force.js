import { defineStore } from 'pinia'

export const useForceStore = defineStore('force', {
  state: () => ({
    k1: {"SEQ01": [0]},
    k2: {"SEQ01": [0]}, 
    k3: {"SEQ01": [0]},
    k4: {"SEQ01": [0]},
    k5: {"SEQ01": [0]},
    k6: {"SEQ01": [0]},
    p1: [],
    p2: [],
    p3: [],
    p4: [],
    p5: [],
    p6: [],
  })
})
