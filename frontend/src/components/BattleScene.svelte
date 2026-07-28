<script>
    import CharacterToken from './CharacterToken.svelte'

    let {
      cardA,
      cardB,
      attackerTeam = null,
      turnKey = 0,
      floatA = null,
      floatB = null,
      floatKindA = 'damage',
      floatKindB = 'damage',
    } = $props()
</script>

<div class="relative w-full aspect-video max-h-105 overflow-hidden rounded-xl border-2 border-(--panel-border)">

        <!-- floor -->
        <div class="absolute inset-0 flex items-center justify-center overflow-hidden" style="perspective: 1600px;
            background: radial-gradient(ellipse at 50% 30%, #23253a 0%, #121421 70%);">
            <div class="w-[250%] h-[250%] shrink-0 border rounded-lg border-(--panel-border)"
                style="
                    background-image:
                        linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px),
                        linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px);
                    background-size: 3.125% 3.875%;
                    background-color: rgba(255,255,255,0.015);
                    transform: rotateX(50deg) rotateZ(135deg) scale(1.15);
                    transform-style: preserve-3d;
                    box-shadow: 0 0 60px rgba(212,175,105,0.05) inset;
                ">
                </div>
        </div>

        <!-- vs -->
        <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
            <span class="text-2xl md:text-3xl" style="color: var(--gold); text-shadow: 0 0 12px rgba(210,175,105,0.25);">VS</span>
        </div>

        {#if cardA}
            <CharacterToken
                name={cardA.name}
                team="A"
                hp={cardA.hp}
                maxHp={cardA.maxHp}
                active={attackerTeam === 'A'}
                corner="left"
                floatKey={turnKey}
                floatText={floatA}
                floatKind={floatKindA}
            />
        {/if}
        {#if cardB}
            <CharacterToken
                name={cardB.name}
                team="B"
                hp={cardB.hp}
                maxHp={cardB.maxHp}
                active={attackerTeam === 'B'}
                corner="right"
                floatKey={turnKey}
                floatText={floatB}
                floatKind={floatKindB}
            />
        {/if}
</div>
