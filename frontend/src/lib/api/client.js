import { translateError } from '../errorMessages.js'

const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

export class ApiError extends Error {
  constructor(kind, message) {
    super(message)
    this.kind = kind
  }
}

function kindFromStatus(status) {
  if (status === 404) return 'not_found'
  if (status === 401) return 'unauthorized'
  if (status === 409 || status === 400) return 'validation'
  return 'server_error'
}

async function buildApiError(res) {
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
  return new ApiError(kindFromStatus(res.status), translateError(body?.error))
}

async function handleResponse(res) {
  if (!res.ok) {
    throw await buildApiError(res)
  }
  if (res.status === 204) return null
  return res.json()
}

export async function apiFetch(path, options = {}) {
  const res = await fetch(`${BASE_URL}${path}`, {
    credentials: 'same-origin',
    ...options,
  })
  return handleResponse(res)
}
