
import { title } from 'process';

<template>
  <div>
    <q-card>
      <q-bar>
        <q-icon name="bi-display" />
        <div>{{ title }}</div>
      </q-bar>

      <q-card-section>
        <canvas :id="`k${id}`"></canvas>
        <span v-if="!isVisible">Not enough data</span>
      </q-card-section>
    </q-card>
  </div>
</template>

<script src="./DaPlot.js"></script>
