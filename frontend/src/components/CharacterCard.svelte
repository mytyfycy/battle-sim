<script>
    let { name, hp, maxHp, strength, defense, spell, statusList = [], highlighted = false } = $props()

    $inspect(statusList)

    const hpPercent = $derived(Math.max(0, Math.min(100, (hp / maxHp) * 100)))
    const hpColor = $derived(
      hpPercent > 50 ? 'bg-emerald-500' : hpPercent > 20 ? 'bg-orange-500' : 'bg-red-600'
    )
</script>

<div
    class="rounded-lg border-2 p-4 bg-gray-700 text-white transition-colors duration-300"
    class:border-teal-500={highlighted}
    class:border-gray-500={!highlighted}
>
    <h2 class="text-lg font-bold mb-2">{name}</h2>

    <div class="mb-2">
        <div class="flex justify-between text-sm mb-1">
            <span>HP</span>
            <span>{hp} / {maxHp}</span>
        </div>
        <div class="w-full h-3 bg-slate-800 rounded overflow-hidden">
            <div class="h-full {hpColor} transition-all duration-500" style="width: {hpPercent}%"></div>
        </div>
    </div>

    <dl class="grid grid-cols-2 gap-x-4 gap-y-1 text-sm mt-3">
        <dt class="text-slate-400">Sila</dt>
        <dd>{strength}</dd>
        <dt class="text-slate-400">Obrona</dt>
        <dd>{defense}</dd>
        <dt class="text-slate-400">Zaklecie</dt>
        <dd>{spell}</dd>
    </dl>

    {#if statusList.length > 0}
        <div class="mt-3 pt-2 border-t border-gray-600 text-xs text-purple-300">
        {#each statusList as status}
            <div>Aktywny status: {JSON.stringify(status.kind)}</div>
        {/each}
        </div>
    {/if}
</div>
