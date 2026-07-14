use serde::Serialize;

#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
pub enum StatusKind {
    ExtraDefenseAura { amount: i32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriggerPoint {
    OnDefend,
}

impl StatusKind {
    fn trigger_point(&self) -> TriggerPoint {
        match self {
            StatusKind::ExtraDefenseAura { .. } => TriggerPoint::OnDefend,
        }
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct StatusEffect {
    pub kind: StatusKind,
    pub turns_remaining: u32,
}

impl StatusEffect {
    pub fn once(kind: StatusKind) -> Self {
        StatusEffect {
            kind,
            turns_remaining: 1,
        }
    }

    fn trigger_point(&self) -> TriggerPoint {
        self.kind.trigger_point()
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct StatusEffects(Vec<StatusEffect>);

impl StatusEffects {
    pub fn new() -> Self {
        StatusEffects(Vec::new())
    }

    pub fn add(&mut self, effect: StatusEffect) {
        self.0.push(effect);
    }

    pub fn consume_defense_bonus(&mut self) -> i32 {
        let mut bonus = 0;

        for effect in self.0.iter_mut() {
            if effect.trigger_point() != TriggerPoint::OnDefend {
                continue;
            }

            let StatusKind::ExtraDefenseAura { amount } = effect.kind;
            bonus += amount;
            effect.turns_remaining = effect.turns_remaining.saturating_sub(1);
        }

        self.0.retain(|e| e.turns_remaining > 0);
        bonus
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
