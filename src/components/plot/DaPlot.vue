<template>
  <div>
    <q-card>
      <q-bar>
        <q-icon name="bi-display" />
        <div>{{ title }}</div>
      </q-bar>

      <q-card-section>
        <Line
          :chart-options="chartOptions"
          :chart-data="chartData"
          :chart-id="chartId"
        />
      </q-card-section>
    </q-card>
  </div>
</template>

<script src="./DaPlot.js"></script>
