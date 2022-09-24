import { defineStore } from 'pinia'

export const useForceStore = defineStore('force', {
  state: () => ({
    k1: {SEQ1: [0, 10, 5, 2, 20, 30, 45]},
    k2: {SEQ1: []},
    k3: {SEQ1: []},
    k4: {SEQ1: []},
    k5: {SEQ1: []},
    k6: {SEQ1: []},
  })
})
