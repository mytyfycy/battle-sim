<script>
    import { onMount, onDestroy } from 'svelte'
    import DataTable from 'datatables.net-dt'
    import 'datatables.net-dt/css/dataTables.dataTables.css'

    let { columns, ajaxUrl, onDetailsClick } = $props()

    let tableEl
    let dt

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
        },
        searching: true,
        ordering: true,
      })

      tableEl.addEventListener('click', handleRowClick)
    })

    onDestroy(() => {
      tableEl?.removeEventListener('click', handleRowClick)
      dt?.destroy()
    })
</script>

<table bind:this={tableEl} class="display w-full"></table>
