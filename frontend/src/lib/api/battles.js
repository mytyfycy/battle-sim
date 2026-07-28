const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

export class ApiError extends Error {
  constructor(kind, message) {
    super(message)
    this.kind = kind
  }
}

async function buildApiError(res) {
  if (res.status === 404) {
    return new ApiError('not_found', 'Nie znaleziono')
  }

  if (res.status === 429) {
    const retryAfter = res.headers.get('Retry-After')
    const seconds = retryAfter ? parseInt(retryAfter, 10) : null
    const message =
      seconds && !Number.isNaN(seconds)
        ? `Sprobuj ponownie za ${seconds}s`
        : `Odczekaj chwile i sprobuj ponownie`
    return new ApiError('rate_limited', message)
  }
  const body = await res.json().catch(() => null)
  return new ApiError('server_error', body?.error ?? 'Blad serwera')
}

async function handleResponse(res) {
  if (!res.ok) {
    throw await buildApiError(res)
  }
  return res.json()
}

export async function startBattle() {
  const res = await fetch(`${BASE_URL}/battles`, { method: 'POST' })
  return handleResponse(res)
}

export async function getBattle(id) {
  const res = await fetch(`${BASE_URL}/battles/${id}`)
  return handleResponse(res)
}
