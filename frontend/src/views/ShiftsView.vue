<script setup lang="ts">
import { User, UserGrade } from '@/types/user.ts'
import UserBadge from '@/components/UserBadge.vue'

function randomAvatar(seed: any) {
  return `https://api.dicebear.com/7.x/identicon/png?seed=${seed}`
}

const users = Object.values(UserGrade).map((grade) => {
  let name = <string>UserGrade[<number>grade]?.toString()
  return new User(name, randomAvatar(name), name)
})
</script>

<template>
  <div class="m4 flex flex-col space-y-3">
    <UserBadge v-for="user in users" v-bind="user" class="m-2" />
  </div>
</template>

<style scoped></style>
