<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getShiftsWeek } from '@/api.ts'
import { DateTime, Interval } from 'luxon'
import { Shift } from '@/types/shift.ts'
import { useToastStore } from '@/stores/toast.ts'
import ShiftCard from '@/components/ShiftCard.vue'

const toast = useToastStore()

let loaded = ref(false)
let shifts = [] as Shift[]

let currentDate = DateTime.now().minus({ days: 1 })

async function getShifts(date: DateTime) {
  loaded.value = false
  await getShiftsWeek(<string>date.toSQLDate())
    .then((res) => {
      const start = date
      const end = date.plus({ days: 7 })
      shifts = []
      Interval.fromDateTimes(start, end)
        .splitBy({ days: 1 })
        .map((d) => <DateTime>d.start)
        .forEach((d) => {
          const noon = d.set({ hour: 12, minute: 0, second: 0, millisecond: 0 })
          const dayShift = res.find(
            (s) =>
              d.toISODate() === s.datetime.toISODate() &&
              s.datetime.toISOTime() === noon.toISOTime(),
          )
          shifts.push(
            dayShift !== undefined ? dayShift : new Shift(<string>noon.toSQLDate(), 'day', []),
          )
          const night = d.set({ hour: 20, minute: 0, second: 0, millisecond: 0 })
          const nightShift = res.find(
            (s) =>
              d.toISODate() === s.datetime.toISODate() &&
              s.datetime.toISOTime() === night.toISOTime(),
          )
          shifts.push(nightShift || new Shift(<string>night.toSQLDate(), 'night', []))
        })
      loaded.value = true
    })
    .catch((e) => toast.error(e))
}

onMounted(async () => {
  await getShifts(currentDate)
})
</script>

<template>
  <ShiftCard v-if="loaded" v-for="shift in shifts" v-bind="shift" />
  <div v-else>Loading...</div>
</template>

<style scoped></style>
