import {
  getMe,
  login as apiLogin,
  logout as apiLogout,
  register as apiRegister
} from './api/auth'

export const authState = $state({
  nick: null,
  ready: false,
})

export async function refreshAuth() {
  try {
    const res = await getMe()
    authState.nick = res.nick
  } catch {
    authState.nick = null
  } finally {
    authState.ready = true
  }
}

export async function login(nick, password) {
  const res = await apiLogin(nick, password)
  authState.nick = res.nick
}

export function register(nick, password) {
  return apiRegister(nick, password)
}

export async function logout() {
  await apiLogout()
  authState.nick = null
}
