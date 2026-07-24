/**
 * @typedef {Object} CombatantSnapshot
 * @property {number} hp
 * @property {number} defense
 * @property {Array} status_list
 *
 * @typedef {Object} TurnLog
 * @property {number} turn_number
 * @property {"A"|"B"} attacker_team
 * @property {number} base_damage
 * @property {number} defender_defense_bonus_consumed
 * @property {string|null} spell_triggered
 * @property {string|null} spell_description
 * @property {number} spell_defense_bonus_promised
 * @property {CombatantSnapshot} character_a_after
 * @property {CombatantSnapshot} character_b_after
 *
 * @typedef {Object} BattleResult
 * @property {Object} character_a_start
 * @property {Object} character_b_start
 * @property {TurnLog[]} turns
 * @property {"A"|"B"} winner_team
 *
 * @typedef {Object} BattleListItem
 * @property {string} id
 * @property {string} character_a_name
 * @property {string} character_b_name
 * @property {string} winner_name
 * @property {string} loser_name
 * @property {number} attacker_hp_at_end
 */

export { }
