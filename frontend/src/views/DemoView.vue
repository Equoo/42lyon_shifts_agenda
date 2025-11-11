<script setup lang="ts">
import { User, UserGrade } from '@/types/user.ts'
import UserBadge from '@/components/UserBadge.vue'

function randomAvatar(seed: any) {
  return `https://api.dicebear.com/7.x/identicon/png?seed=${seed}`
}

const users = Object.values(UserGrade)
  .map((grade) => <string>UserGrade[<number>grade]?.toString())
  .filter((name) => name.length > 1)
  .map((name) => new User(name, randomAvatar(name), name))
</script>

<template>
  <div class="m-4">
    <UserBadge v-for="user in users" v-bind="user" class="m-2" />
  </div>
</template>

<style scoped></style>
