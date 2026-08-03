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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let status_list = StatusEffects::new();
        assert!(status_list.is_empty());
    }

    #[test]
    fn consume_defense_bonus_sums_multiple_effects() {
        let mut status_list = StatusEffects::new();

        status_list.add(StatusEffect::once(StatusKind::ExtraDefenseAura {
            amount: 5,
        }));
        status_list.add(StatusEffect::once(StatusKind::ExtraDefenseAura {
            amount: 3,
        }));

        let bonus = status_list.consume_defense_bonus();

        assert_eq!(bonus, 8);
    }

    #[test]
    fn consume_defense_bonus_returns_zero_when_empty() {
        let mut status_list = StatusEffects::new();
        assert_eq!(status_list.consume_defense_bonus(), 0);
    }

    #[test]
    fn effect_expires_after_being_consumed_once() {
        let mut status_list = StatusEffects::new();

        status_list.add(StatusEffect::once(StatusKind::ExtraDefenseAura {
            amount: 5,
        }));

        let first = status_list.consume_defense_bonus();
        assert_eq!(first, 5);
        assert!(status_list.is_empty());

        let second = status_list.consume_defense_bonus();
        assert_eq!(second, 0);
    }
}
