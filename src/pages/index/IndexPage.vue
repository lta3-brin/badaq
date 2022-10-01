<template>
  <q-page padding>
    <div class="row items-center justify-center q-col-gutter-md">
      <div class="col-12 col-md-4">
        <q-card>
          <q-bar>
            <q-icon name="bi-display" />
            <div>Fx (N)</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptionsFx"
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
            <div>Fy (N)</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptionsFy"
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
            <div>Fz (N)</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptionsFz"
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
            <div>Mx (Nm)</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptionsMx"
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
            <div>My (Nm)</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptionsMy"
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
            <div>Mz (Nm)</div>
          </q-bar>

          <q-card-section>
            <Line
              :chart-options="chartOptionsMz"
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
