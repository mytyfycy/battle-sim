import { describe, it, expect, vi, afterEach } from 'vitest'
import { apiFetch, ApiError } from './client'

function mockResponse({ ok, status, jsonBody, headers = {} }) {
  return {
    ok,
    status,
    headers: {
      get: (key) => headers[key] ?? null,
    },
    json: () => (jsonBody === undefined ? Promise.reject(new Error('no body')) : Promise.resolve(jsonBody)),
  }
}

afterEach(() => {
  vi.restoreAllMocks()
  vi.unstubAllGlobals()
})

describe('apiFetch success paths', () => {
  it('returns parsed JSON body on a 200 response', async () => {
    const res = mockResponse({ ok: true, status: 200, jsonBody: { nick: 'valid_nick' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    const result = await apiFetch('/auth/me')

    expect(result).toEqual({ nick: 'valid_nick' })
  })

  it('returns null for a 204 No Content response without reading the body', async () => {
    const res = mockResponse({ ok: true, status: 204 })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    const result = await apiFetch('/auth/logout', { method: 'POST' })

    expect(result).toBeNull()
  })
})

describe('apiFetch error mapping', () => {
  it('maps 404 to "not_found" ApiError with a translated message', async () => {
    const res = mockResponse({ ok: false, status: 404, jsonBody: { error: 'Battle not found' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    await expect(apiFetch('/battles/xyz')).rejects.toMatchObject({
      kind: 'not_found',
      message: 'Nie znaleziono walki',
    })
  })

  it('maps 401 to an "unauthorized" ApiError', async () => {
    const res = mockResponse({ ok: false, status: 401, jsonBody: { error: 'Invalid nick or password' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    await expect(apiFetch('/auth/login')).rejects.toMatchObject({ kind: 'unauthorized' })
  })

  it('maps 400 to a "validation" ApiError', async () => {
    const res = mockResponse({ ok: false, status: 400, jsonBody: { error: 'Invalid request body' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))
    await expect(apiFetch('/auth/register')).rejects.toMatchObject({ kind: 'validation' })
  })

  it('maps 409 to a "validation" ApiError', async () => {
    const res = mockResponse({ ok: false, status: 409, jsonBody: { error: 'Nick is already taken' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))
    await expect(apiFetch('/auth/register')).rejects.toMatchObject({ kind: 'validation' })
  })

  it('maps any other error status to "server_error"', async () => {
    const res = mockResponse({ ok: false, status: 500, jsonBody: { error: 'Internal server error' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    await expect(apiFetch('/battles')).rejects.toMatchObject({ kind: 'server_error' })
  })

  it('falls back to a generic translated message when the error body cannot be parsed', async () => {
    const res = mockResponse({ ok: false, status: 500, jsonBody: undefined })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    await expect(apiFetch('/battles')).rejects.toMatchObject({
      message: 'Wystapil nieoczekiwany blad',
    })
  })

  it('throws instances of ApiError', async () => {
    const res = mockResponse({ ok: false, status: 404, jsonBody: { error: 'Battle not found' } })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    try {
      await apiFetch('/battles/xyz')
      throw new Error('should have thrown')
    } catch (err) {
      expect(err).toBeInstanceOf(ApiError)
    }
  })
})

describe('apiFetch rate limiting 429', () => {
  it('reports the retry delay in seconds when Retry-After is present', async () => {
    const res = mockResponse({
      ok: false,
      status: 429,
      headers: { 'Retry-After': '30' },
    })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    await expect(apiFetch('/battles')).rejects.toMatchObject({
      kind: 'rate_limited',
      message: 'Sprobuj ponownie za 30s',
    })
  })

  it('falls back to a generic wait message when Retry-After is missing or invalid', async () => {
    const res = mockResponse({ ok: false, status: 429, headers: {} })
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(res))

    await expect(apiFetch('/battles')).rejects.toMatchObject({
      kind: 'rate_limited',
      message: 'Odczekaj chwile i sprobuj ponownie',
    })
  })
})
