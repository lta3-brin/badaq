import { defineStore } from 'pinia'

export const useForceStore = defineStore('force', {
  state: () => ({
    sec: 1,
    lbl: [],
    k1: {"SEQ01": []},
    k2: {"SEQ01": []}, 
    k3: {"SEQ01": []},
    k4: {"SEQ01": []},
    k5: {"SEQ01": []},
    k6: {"SEQ01": []},
    p1: [],
    p2: [],
    p3: [],
    p4: [],
    p5: [],
    p6: [],
  })
})
