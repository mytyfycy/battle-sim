<script>
    import { login, register } from '../lib/auth.svelte.js'
    import { ApiError } from '../lib/api/client.js'

    let { navigate } = $props()

    let mode = $state('login')
    let nick = $state('')
    let password = $state('')
    let repeatPassword = $state('')
    let error = $state(null)
    let info = $state(null)
    let submitting = $state(false)

    function switchMode(next) {
      mode = next
      error = null
      info = null
      password = ''
      repeatPassword = ''
    }

    async function handleSubmit() {
      error = null
      info = null

      if (mode === 'register' && password !== repeatPassword) {
        error = 'Hasla nie sa takie same'
        return
      }

      submitting = true
      try {
        if (mode === 'login') {
          await login(nick, password)
          navigate('#/battle')
        } else {
          await register(nick, password)
          switchMode('login')
          info = 'Konto utworzone, mozesz sie teraz zalogowac'
        }
      } catch (err) {
        error = err instanceof ApiError ? err.message : 'Nieoczekiwany blad'
      } finally {
        submitting = false
      }
    }
</script>

<div class="max-w-sm mx-auto px-4 py-8">
    <div class="p-6 rounded-xl border-2 bg-(--panel) border-(--panel-border)">
        <h1 class="text-xl font-semibold text-(--ink) mb-6 text-center">
            {mode === 'login' ? 'Zaloguj sie' : 'Zaloz konto'}
        </h1>

        {#if error}
            <p class="bg-red-500 text-black p-4 rounded-xl font-medium">{error}</p>
        {/if}
        {#if info}
            <p class="bg-(--emerald) text-black p-3 rounded-lg mb-4 text-sm font-medium">{info}</p>
        {/if}

        <form class="flex flex-col gap-4" onsubmit={(e) => { e.preventDefault(); handleSubmit() }}>
            <label class="flex flex-col gap-1 text-sm text-(--ink-dim)">
                Nick
                <input
                    type="text"
                    bind:value={nick}
                    required
                    minlength="3"
                    maxlength="32"
                    class="px-3 py-2 rounded-lg bg-transparent border border-(--panel-border) text-(--ink) focus:outline-none focus:border-(--gold)"
                />
            </label>

            <label class="flex flex-col gap-1 text-sm text-(--ink-dim)">
                Haslo
                <input
                    type="password"
                    bind:value={password}
                    required
                    minlength="8"
                    class="px-3 py-2 rounded-lg bg-transparent border border-(--panel-border) text-(--ink) focus:outline-none focus:border-(--gold)"
                />
            </label>

            {#if mode === 'register'}
                <label class="flex flex-col gap-1 text-sm text-(--ink-dim)">
                    Powtorz haslo
                    <input
                        type="password"
                        bind:value={repeatPassword}
                        required
                        minlength="8"
                        class="px-3 py-2 rounded-lg bg-transparent border border-(--panel-border) text-(--ink) focus:outline-none focus:border-(--gold)"
                    />
                </label>
            {/if}

            <button
                type="submit"
                disabled={submitting}
                class="mt-2 px-6 py-2 bg-(--gold) hover:text-(--ink) rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
            >
                {submitting ? 'Ladowanie...' : mode === 'login' ? 'Zaloguj' : 'Zarejestruj'}
            </button>
        </form>

        <p class="text-center text-sm text-(--ink-dim) mt-4">
            {#if mode === 'login'}
                Nie masz konta?
                <button type="button" onclick={() => switchMode('register')} class="text-(--gold) hover:underline cursor-pointer">Zarejestruj</button>
            {:else}
                Masz juz konto?
                <button type="button" onclick={() => switchMode('login')} class="text-(--gold) hover:underline cursor-pointer">Zaloguj</button>
            {/if}
        </p>
    </div>
</div>
