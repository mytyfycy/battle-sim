CREATE TABLE battles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    character_a_name TEXT NOT NULL,
    character_b_name TEXT NOT NULL,
    winner_name TEXT NOT NULL,
    loser_name TEXT NOT NULL,
    attacker_hp_at_end INTEGER NOT NULL,
    full_result JSONB NOT NULL
);

CREATE INDEX idx_battles_created_at ON battles (created_at);
