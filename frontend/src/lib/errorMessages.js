const ERROR_MESSAGES = {
  'Nick is already taken': 'Nick jest juz zajety',
  'Invalid nick or password': 'Nieprawidlowy nick lub haslo',
  'Nick must be between 3 and 32 characters': 'Nick musi miec od 3 do 32 znakow',
  'Password must be between 8 and 64 characters': 'Haslo musi miec od 8 do 64 znakow',
  'Battle not found': 'Nie zaleziono walki',
  'Internal server error': 'Wystapil blad serwera',
  'Invalid request body': 'Nieprawidlowe dane w zapytaniu',
}

export function translateError(message) {
  return ERROR_MESSAGES[message] ?? 'Wystapil nieoczekiwany blad'
}
