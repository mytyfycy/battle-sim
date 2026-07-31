import { apiFetch } from './client'

export { ApiError } from './client'

export async function startBattle() {
  return apiFetch('/battles', { method: 'POST' })
}

export async function getBattle(id) {
  return apiFetch(`/battles/${id}`)
}
