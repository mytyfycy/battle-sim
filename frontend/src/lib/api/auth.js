import { apiFetch } from './client'

export async function register(nick, password) {
  return apiFetch('/auth/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ nick, password }),
  })
}

export async function login(nick, password) {
  return apiFetch('/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ nick, password }),
  })
}

export async function logout() {
  return apiFetch('/auth/logout', { method: 'POST' })
}

export async function getMe() {
  return apiFetch('/auth/me')
}
