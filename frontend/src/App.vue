<script setup lang="ts"></script>

<template>
  <Suspense>
    <router-view />
    <template #fallback>
      <div>Logging in or smth idk</div>
    </template>
  </Suspense>
</template>

<style scoped></style>
