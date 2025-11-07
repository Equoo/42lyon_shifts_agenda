import axios from 'axios'
import { User } from '@/types/user.ts'

const clientConfig = {
  baseURL: window.location.protocol + '//' + window.location.host + '/api/',
  timeout: 5000,
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

export async function login(): Promise<string> {
  return await client.post('auth/42/login').then((response) => response.data)
}

export async function logout(): Promise<void> {
  return await client.post('auth/logout')
}

export async function handleLoginCallback(code: String): Promise<void> {
  return await client.post('auth/42/callback', {
    code: code,
  })
}
