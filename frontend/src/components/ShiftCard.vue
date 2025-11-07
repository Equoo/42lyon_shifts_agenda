<script setup lang="ts">
const slot = 'wawa'
const date = 'today'
</script>

<template>
  <div class="shift">
    <div class="header">
      <div class="flex items-center gap-2">
        <span class="material-symbols-outlined">
          {{ slot }}
        </span>
        <div class="font-medium">
          {{ slot }} - {{ date }}
          <span>wawa</span>
        </div>
      </div>
      <div class="flex -space-x-1 overflow-visible"></div>
    </div>
  </div>
</template>

<style scoped></style>
