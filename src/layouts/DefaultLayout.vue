<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar class="bg-grey-9 text-white">
        <q-toolbar-title>
          Dashboard
        </q-toolbar-title>

        <div>
          <q-btn flat round icon="bi-info-circle-fill" @click="showDialog = true" />

          <q-dialog v-model="showDialog" persistent>
            <q-card>
              <q-bar>
                <q-icon name="bi-pin-angle-fill" color="negative" />
                <div>About BADAQ Monitor</div>

                <q-space />

                <q-btn dense flat icon="close" v-close-popup>
                  <q-tooltip>Close</q-tooltip>
                </q-btn>
              </q-bar>

              <q-card-section>
                <span class="text-bold">Balance Data Acquisition and Processing System (BADAQ)</span>
                is a six-component balance data acquisition and processing system specially designed
                for wind tunnel experiment needs. This system accommodates data acquisition and
                processing for axial force, side force, normal force, lift moment, yaw moment, and
                rolling moment.
              </q-card-section>
            </q-card>
          </q-dialog>
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script src="./DefaultLayout.js"></script>
