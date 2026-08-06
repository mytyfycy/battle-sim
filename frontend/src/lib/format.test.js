import { describe, expect, it } from 'vitest'
import { truncateNick } from './format.js'

describe('truncateNick', () => {
  it('returns short nicks unchanged', () => {
    expect(truncateNick('Test')).toBe('Test')
  })

  it('returns nick unchanged when exactly at the limit', () => {
    expect(truncateNick('123456789012', 12)).toBe('123456789012')
  })

  it('shortens long nicks with a middle ellipsis', () => {
    expect(truncateNick('BardzoDlugiNickGracza')).toBe('Bard..acza')
  })

  it('keeps the truncated output at or under the max length', () => {
    const result = truncateNick('BardzoDlugiNickGracza', 12)
    expect(result.length).toBeLessThanOrEqual(12)
  })

  it('handles empty/null input', () => {
    expect(truncateNick('')).toBe('')
    expect(truncateNick(null)).toBe(null)
  })
})
