<script setup lang="ts">
import Toaster from '@/components/Toaster.vue'
import NavBar from '@/components/NavBar.vue'
</script>

<template>
  <NavBar />
  <Toaster />
  <router-view />
</template>

<style scoped></style>
