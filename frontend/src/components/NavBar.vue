<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth.ts'
import { ref } from 'vue'
import type { User } from '@/types/user.ts'
import UserBadge from '@/components/UserBadge.vue'

const currentRoute = useRoute();
const auth = useAuthStore();

const routes = [
  {
    name: 'Home',
    path: '/',
  },
  {
    name: 'Shifts',
    path: '/shifts',
  },
]

const me = ref(null as User | null);
auth.$subscribe(() => me.value = auth.user)

</script>

<template>
  <nav class="sticky top-0 w-full bg-gray-100 dark:bg-gray-800 after:pointer-events-none after:absolute after:inset-x-0 after:bottom-0 after:h-px after:bg-black/5 after:dark:bg-white/10">
    <div class="mx-auto max-w-7xl px-6">
      <div class="relative flex h-16 items-center justify-between">
        <div class="flex flex-1 items-stretch justify-start">
          <div class="flex shrink-0 items-center">
            <RouterLink to="/">
              <img src="/logo.png" alt="42 EAT Logo" class="h-8" />
            </RouterLink>
          </div>
          <div class="ml-6 block">
            <div class="flex space-x-4">
              <RouterLink
                v-for="route in routes"
                :to="route.path"
                :class="(route.path === currentRoute.path ? 'bg-gray-950/40' : 'hover:bg-white/5 hover:text-white')"
                class="rounded-md px-3 py-2 text-md font-medium text-gray-300 transition-colors duration-300"
              >
                {{ route.name }}
              </RouterLink>
            </div>
          </div>
        </div>
        <div class="static inset-y-0 right-0 flex items-center inset-auto ml-6 pr-0">
          <UserBadge v-if="me" v-bind="<User>me" />
          <div v-else>Not logged in</div>
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped></style>
