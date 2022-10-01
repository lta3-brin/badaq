import { defineStore } from 'pinia'

export const useForceStore = defineStore('force', {
  state: () => ({
    connected: false,
    sec: 1,
    lbl: [],
    p1: [],
    p2: [],
    p3: [],
    p4: [],
    p5: [],
    p6: [],
  })
})
