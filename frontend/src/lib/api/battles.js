const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

async function handleResponse(res) {
  if (res.status === 404) {
    throw new ApiError('not_found', 'Nie znaleziono')
  }
  if (!res.ok) {
    const body = await res.json().catch(() => null)
    throw new ApiError('server_error', body?.error ?? 'Blad serwera')
  }
  return res.json()
}

export class ApiError extends Error {
  constructor(kind, message) {
    super(message)
    this.kind = kind
  }
}

export async function startBattle() {
  const res = await fetch(`${BASE_URL}/battles`, { method: 'POST' })
  return handleResponse(res)
}

export async function getBattle(id) {
  const res = await fetch(`${BASE_URL}/battles/${id}`)
  return handleResponse(res)
}
