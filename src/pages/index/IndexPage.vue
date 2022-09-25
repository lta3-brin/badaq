<template>
  <q-page padding>
    <div class="row items-center justify-center q-col-gutter-md">
      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>K1</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptions"
              :chart-data="chartData1"
              chart-id="k1"
              :height="180"
            />
          </q-card-section>
        </q-card>
      </div>
      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>K2</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptions"
              :chart-data="chartData2"
              chart-id="k2"
              :height="180"
            />
          </q-card-section>
        </q-card>
      </div>
      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>K3</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptions"
              :chart-data="chartData3"
              chart-id="k3"
              :height="180"
            />
          </q-card-section>
        </q-card>
      </div>

      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>K4</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptions"
              :chart-data="chartData4"
              chart-id="k4"
              :height="180"
            />
          </q-card-section>
        </q-card>
      </div>

      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>K5</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptions"
              :chart-data="chartData5"
              chart-id="k5"
              :height="180"
            />
          </q-card-section>
        </q-card>
      </div>

      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>K6</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptions"
              :chart-data="chartData6"
              chart-id="k6"
              :height="180"
            />
          </q-card-section>
        </q-card>
      </div>
    </div>
  </q-page>
</template>

<script src="./IndexPage.js">
</script>
