export function truncateNick(nick, maxLength = 10) {
  if (!nick) return nick
  if (nick.length <= maxLength) return nick

  const keep = Math.max(1, Math.floor((maxLength - 2) / 2))
  return `${nick.slice(0, keep)}..${nick.slice(-keep)}`
}
