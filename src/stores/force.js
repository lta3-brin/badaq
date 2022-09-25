import { defineStore } from 'pinia'

export const useForceStore = defineStore('force', {
  state: () => ({
    sec: 1,
    lbl: [],
    k1: {"SEQ1": []},
    k2: {"SEQ1": []}, 
    k3: {"SEQ1": []},
    k4: {"SEQ1": []},
    k5: {"SEQ1": []},
    k6: {"SEQ1": []},
    p1: [],
    p2: [],
    p3: [],
    p4: [],
    p5: [],
    p6: [],
  })
})
