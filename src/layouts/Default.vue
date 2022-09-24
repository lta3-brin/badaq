<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar class="bg-grey-9 text-white">
        <q-toolbar-title>
          Dashboard
        </q-toolbar-title>

        <div>
          <q-btn flat round icon="bi-info-circle-fill" />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script src="./Default.js"></script>
