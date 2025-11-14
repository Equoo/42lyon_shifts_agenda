import axios from 'axios'
import { User } from '@/types/user.ts'
import { Shift } from '@/types/shift.ts'

const clientConfig = {
  baseURL: window.location.protocol + '//' + window.location.host + '/api/',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
  withCredentials: true,
}
const client = axios.create(clientConfig)

export async function getMe(): Promise<User> {
  console.log('calling me')
  console.log(clientConfig)
  return await client
    .get('auth/me')
    .then((response) => response.data)
    .then((data) => new User(data.login, data.img_url, data.grade))
}

export async function getShiftsWeek(date: string): Promise<Shift[]> {
  return await client
    .get(`shifts/week?start=${date}`)
    .then((response) => response.data as any[])
    .then((data) =>
      data.map((s) => {
        const users = (s.users as any[]).map((u) => new User(u.login, u.img_url, u.grade))
        return new Shift(s.date, s.slot, users)
      }),
    )
}

export async function registerToShift(date: string, slot: string): Promise<void> {
  return await client.post('shifts/register', { date, slot })
}

export async function unregisterFromShift(date: string, slot: string): Promise<void> {
  return await client.post('shifts/unregister', { date, slot })
}

export async function apiLogin(): Promise<string> {
  return await client.post('auth/42/login').then((response) => response.data)
}

export async function apiLogout(): Promise<void> {
  return await client.post('auth/logout')
}

export async function handleLoginCallback(code: String): Promise<void> {
  return await client.post('auth/42/callback', {
    code: code,
  })
}
