<script>
    import DataTable from "../components/DataTable.svelte"

    let { navigate } = $props()

    const columns = [
      { title: 'ID walki', data: 'id' },
      { title: 'Gracz A', data: 'character_a_name' },
      { title: 'Gracz B', data: 'character_b_name' },
      { title: 'Wygrany', data: 'winner_name' },
      { title: 'Przegrany', data: 'loser_name' },
      { title: 'HP atakujacego na koniec', data: 'attacker_hp_at_end', className: 'dt-center'},
      {
        title: 'Szczegoly',
        data: 'id',
        orderable: false,
        render: (id) => `
          <button
            data-battle-id="${id}"
            class="px-6 py-3 bg-teal-500 hover:bg-teal-800 text-white rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
            >
              Zobacz
            </button>
          `,
      },
    ]

    function goToDetails(id) {
      navigate(`#/history/${id}`)
    }
</script>

<div class="max-w-6xl mx-auto">
    <div class="px-6 py-3 text-center text-white bg-gray-700 border rounded-xl">
        <DataTable {columns} ajaxUrl="http://localhost:3000/battles" onDetailsClick={goToDetails} />
    </div>
</div>
