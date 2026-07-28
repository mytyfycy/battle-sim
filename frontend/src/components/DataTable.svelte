<script>
    import { onMount, onDestroy } from 'svelte'
    import DataTable from 'datatables.net-dt'
    import 'datatables.net-dt/css/dataTables.dataTables.css'

    let { columns, ajaxUrl, onDetailsClick } = $props()

    let tableEl
    let dt
    let loadError = $state(null)

    function handleRowClick(e) {
      const btn = e.target.closest('[data-battle-id]')
      if (btn) {
        onDetailsClick?.(btn.dataset.battleId)
      }
    }

    onMount(() => {
        dt = new DataTable(tableEl, {
          columns,
          serverSide: true,
          processing: true,
          ajax: {
            url: ajaxUrl,
            type: 'GET',
            error: (xhr) => {
              if (xhr.status === 429) {
                const retryAfter = xhr.getResponseHeader('Retry-After')
                const seconds = retryAfter ? parseInt(retryAfter, 10) : null
                loadError = seconds && !Number.isNaN(seconds)
                            ? `Sprobuj ponownie za ${seconds}s`
                            : 'Odczekaj chwile i sprobuj ponowie'
              } else {
                loadError = 'Nie udalo sie wczytac historii walk'
              }
              dt?.processing(false)
            },
          },
          searching: true,
          ordering: true,
        })

        dt.on('xhr.dt', (e, settings, json) => {
          if (json) loadError = null
        })

        tableEl.addEventListener('click', handleRowClick)
    })

    onDestroy(() => {
      tableEl?.removeEventListener('click', handleRowClick)
      dt?.destroy()
    })
</script>

{#if loadError}
    <p class="bg-red-500 text-black p-4 rounded-xl font-medium">Blad: {loadError}</p>
{/if}

<div style="display: {loadError ? 'none' : 'block' };">
    <table bind:this={tableEl} class="display w-full"></table>
</div>
