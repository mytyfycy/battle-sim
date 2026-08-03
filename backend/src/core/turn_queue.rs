use anyhow::{Result, bail};
use rand::Rng;
use serde::Serialize;

pub const BASE_DELAY: i32 = 100;

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamId {
    A,
    B,
}

impl TeamId {
    pub fn opponent(self) -> TeamId {
        match self {
            TeamId::A => TeamId::B,
            TeamId::B => TeamId::A,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CombatantId(pub u32);

#[derive(Clone, Copy, Debug)]
struct Combatant {
    id: CombatantId,
    team: TeamId,
    delay: i32,
}

#[derive(Debug, Clone)]
pub struct TurnQueue {
    combatant_a: Combatant,
    combatant_b: Combatant,
    last_attacker: Option<CombatantId>,
}

impl TurnQueue {
    pub fn new_1vs1(
        combatant_a: CombatantId,
        combatant_b: CombatantId,
        rng: &mut impl Rng,
    ) -> Self {
        let a_first = rng.gen_bool(0.5);

        TurnQueue {
            combatant_a: Combatant {
                id: combatant_a,
                team: TeamId::A,
                delay: if a_first { 0 } else { BASE_DELAY },
            },
            combatant_b: Combatant {
                id: combatant_b,
                team: TeamId::B,
                delay: if a_first { BASE_DELAY } else { 0 },
            },
            last_attacker: None,
        }
    }

    pub fn current_attacker(&self) -> CombatantId {
        use std::cmp::Ordering;

        match self.combatant_a.delay.cmp(&self.combatant_b.delay) {
            Ordering::Less => self.combatant_a.id,
            Ordering::Greater => self.combatant_b.id,
            Ordering::Equal => {
                if self.last_attacker == Some(self.combatant_a.id) {
                    self.combatant_b.id
                } else {
                    self.combatant_a.id
                }
            }
        }
    }

    pub fn team_of(&self, id: CombatantId) -> Result<TeamId> {
        Ok(self.find(id)?.team)
    }

    pub fn advance_after_turn(&mut self, attacker: CombatantId) -> Result<()> {
        self.set_delay(attacker, BASE_DELAY)?;
        self.last_attacker = Some(attacker);
        Ok(())
    }

    pub fn grant_extra_turn(&mut self, attacker: CombatantId) -> Result<()> {
        self.set_delay(attacker, 0)?;
        self.last_attacker = Some(attacker);
        Ok(())
    }

    fn set_delay(&mut self, id: CombatantId, value: i32) -> Result<()> {
        let combatant = self.find_mut(id)?;
        combatant.delay = value.max(0);
        Ok(())
    }

    fn find_mut(&mut self, id: CombatantId) -> Result<&mut Combatant> {
        if self.combatant_a.id == id {
            Ok(&mut self.combatant_a)
        } else if self.combatant_b.id == id {
            Ok(&mut self.combatant_b)
        } else {
            bail!("Unknown CombatantId: {id:?}")
        }
    }

    fn find(&self, id: CombatantId) -> Result<&Combatant> {
        if self.combatant_a.id == id {
            Ok(&self.combatant_a)
        } else if self.combatant_b.id == id {
            Ok(&self.combatant_b)
        } else {
            bail!("Unknown CombatantId: {id:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    const A: CombatantId = CombatantId(0);
    const B: CombatantId = CombatantId(1);

    // StepRng always returning 0 forces every gen_bool call to true
    // any probability > 0 passes
    // u64::MAX forces false
    fn always_true_rng() -> StepRng {
        StepRng::new(0, 0)
    }

    fn always_false_rng() -> StepRng {
        StepRng::new(u64::MAX, 0)
    }

    #[test]
    fn a_goes_first_when_coin_flip_is_true() {
        let mut rng = always_true_rng();
        let q = TurnQueue::new_1vs1(A, B, &mut rng);

        assert_eq!(q.current_attacker(), A);
        assert_eq!(q.team_of(A).unwrap(), TeamId::A);
    }

    #[test]
    fn b_goes_first_when_coin_flip_is_false() {
        let mut rng = always_false_rng();
        let q = TurnQueue::new_1vs1(A, B, &mut rng);

        assert_eq!(q.current_attacker(), B);
        assert_eq!(q.team_of(B).unwrap(), TeamId::B);
    }

    #[test]
    fn advance_after_turn_passes_the_turn_to_the_other_combatant() {
        let mut rng = always_true_rng();
        let mut q = TurnQueue::new_1vs1(A, B, &mut rng);

        assert_eq!(q.current_attacker(), A);
        q.advance_after_turn(A).unwrap();

        // Both delays are equal
        // last_attacker was A
        // B should attack next
        assert_eq!(q.current_attacker(), B);
    }

    #[test]
    fn grant_extra_turn_lets_the_same_combatant_act_again() {
        let mut rng = always_true_rng();
        let mut q = TurnQueue::new_1vs1(A, B, &mut rng);

        assert_eq!(q.current_attacker(), A);
        q.grant_extra_turn(A).unwrap();

        assert_eq!(q.current_attacker(), A);
    }

    #[test]
    fn turns_alternate_over_several_rounds() {
        let mut rng = always_true_rng();
        let mut q = TurnQueue::new_1vs1(A, B, &mut rng);

        let mut attackers = Vec::new();
        for _ in 0..4 {
            let attacker = q.current_attacker();
            attackers.push(attacker);
            q.advance_after_turn(attacker).unwrap();
        }

        assert_eq!(attackers, vec![A, B, A, B]);
    }

    #[test]
    fn team_of_unknown_combatant_errors() {
        let mut rng = always_true_rng();
        let q = TurnQueue::new_1vs1(A, B, &mut rng);

        let unknown = CombatantId(100);
        assert!(q.team_of(unknown).is_err());
    }

    #[test]
    fn advance_after_turn_unknown_combatant_errors() {
        let mut rng = always_true_rng();
        let mut q = TurnQueue::new_1vs1(A, B, &mut rng);

        let unknown = CombatantId(100);
        assert!(q.advance_after_turn(unknown).is_err());
    }

    #[test]
    fn opponent_returns_the_other_team() {
        assert_eq!(TeamId::A.opponent(), TeamId::B);
        assert_eq!(TeamId::B.opponent(), TeamId::A);
    }
}
