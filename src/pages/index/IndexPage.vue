<template>
  <q-page padding>
    <div class="row items-center justify-center q-col-gutter-md">
      <div class="col-12 col-md-4">
        <Line
          :chart-options="chartOptions"
          :chart-data="chartData"
          chart-id="k1"
          :height="180"
        />
      </div>
      <div class="col-12 col-md-4">
        
      </div>
      <div class="col-12 col-md-4">
        One of three cols
      </div>
    </div>
  </q-page>
</template>

<script src="./IndexPage.js">
</script>
