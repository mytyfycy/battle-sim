import { describe, it, expect } from 'vitest'
import { translateError } from './errorMessages.js'

describe('translateError', () => {
  it('translates known backend error messages to Polish', () => {
    expect(translateError('Nick is already taken')).toBe('Nick jest juz zajety')
    expect(translateError('Invalid nick or password')).toBe('Nieprawidlowy nick lub haslo')
    expect(translateError('Nick must be between 3 and 32 characters')).toBe('Nick musi miec od 3 do 32 znakow')
    expect(translateError('Password must be between 8 and 64 characters')).toBe('Haslo musi miec od 8 do 64 znakow')
    expect(translateError('Battle not found')).toBe('Nie zaleziono walki')
    expect(translateError('Internal server error')).toBe('Wystapil blad serwera')
    expect(translateError('Invalid request body')).toBe('Nieprawidlowe dane w zapytaniu')
  })

  it('falls back to a generic message for an unknown error', () => {
    expect(translateError('Something the backend never sends')).toBe(
      'Wystapil nieoczekiwany blad'
    )
  })

  it('falls back to a generic message for undefined/null input', () => {
    expect(translateError(undefined)).toBe('Wystapil nieoczekiwany blad')
    expect(translateError(null)).toBe('Wystapil nieoczekiwany blad')
  })
})
