<script>
    let {
      name,
      team,
      hp,
      maxHp,
      active = false,
      corner = 'left',
      floatKey = 0,
      floatText = null,
      floatKind = 'damage',
    } = $props()

    const hpPercent = $derived(Math.max(0, Math.min(100, (hp / maxHp) * 100)))
    const teamColor = $derived(team === 'A' ? 'var(--teal)' : 'var(--crimson)')
    const positionClass = $derived(corner === 'left' ? 'left-[10%] bottom-[50%]' : 'right-[10%] bottom-[50%]')
    const floatColor = $derived(
      floatKind === 'heal' ? 'var(--emerald)'
      : floatKind === 'buff' ? 'var(--gold)'
      : 'var(--crimson)')
</script>

<div class="absolute {positionClass} flex flex-col items-center" style="--tx:0px; --ty:0px">

    <!-- Floating text -->
    <div class="relative h-8 w-full flex justify-center">
        {#key floatKey}
            {#if floatText}
                <span
                    class="absolute font-bold text-sm whitespace-nowrap animate-float-up"
                    style="color: {floatColor}; text-shadow: 0 0 5px rgba(0,0,0,0.9);"
                >{floatText}</span>
            {/if}
        {/key}
    </div>

    <!-- Name -->
    <div class="text-sm md:text-base mb-1 px-2 py-0.5 rounded bg-black border"
        style="
        border-color: {active ? 'var(--gold)' : 'var(--panel-border)'};
        color: var(--ink);
        ">
        {name}
    </div>

    <!-- Diamond -->
    <div class="relative w-16 h-16 md:w-20 md:h-20 my-3 md:my-4"
        class:animate-attack={active}
        style="--dx: {corner === 'left' ? '26px' : '-26px'};
        --dy: -10px;
        ">
        <div class="absolute inset-0 rotate-45 rounded-md border-2 shadow-lg"
            style="
            background: linear-gradient(135deg, {teamColor}, #0000 70%), var(--panel);
            border-color: {active ? 'var(--gold)' : 'var(--panel-border)'};
            {active ? 'animation: pulse-glow 1s ease-in-out infinite;' : ''}
            ">
        </div>
        <div class="absolute inset-0 flex items-center justify-center font-bold text-2xl" style="color: {teamColor};">
            {name.charAt(0)}
        </div>
    </div>

    <!-- HP -->
    <div class="mt-2 w-20 md:w-24 h-2 rounded-full overflow-hidden bg-black border border-(--panel-border)">
        <div class="h-full transition-all duration-250"
            style="width: {hpPercent}%; background:
            {hpPercent > 50 ? 'var(--emerald)' : hpPercent > 20 ? 'var(--gold)' : 'var(--crimson)'};"
        >
        </div></div>
    <div>{Math.max(0, hp)} / {maxHp}</div>

    <div></div>
</div>
