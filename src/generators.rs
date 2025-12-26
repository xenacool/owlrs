//! # Proptest Generators: Creating Random Narrative Scenarios
//!
//! This module provides proptest strategies for generating random but valid
//! narrative scenarios that can be tested against our property invariants.
//!
//! ## The Power of Generative Testing
//!
//! Instead of manually writing test scenarios like:
//! ```text
//! "Character A meets Character B, then B dies, then A references B"
//! ```
//!
//! We generate thousands of random scenarios:
//! ```text
//! Random sequence of 50 events involving 13 characters across 5 timelines,
//! including memory trades, deaths, resurrections, relationship changes...
//! ```
//!
//! And check that ALL properties hold for ALL generated scenarios.
//!
//! When a property fails, proptest **shrinks** the failing case to the minimal
//! reproduction—just like TLA+ counterexamples, but much faster.

use crate::narrative_core::*;
use proptest::prelude::*;

/// Strategy for generating TimelineIds
pub fn timeline_id_strategy() -> impl Strategy<Value = TimelineId> {
    (0u64..10).prop_map(TimelineId)
}

/// Strategy for generating CharacterIds
pub fn character_id_strategy() -> impl Strategy<Value = CharacterId> {
    (0u64..13).prop_map(CharacterId) // 13 protagonists!
}

/// Strategy for generating MemoryIds
pub fn memory_id_strategy() -> impl Strategy<Value = MemoryId> {
    any::<u64>().prop_map(MemoryId)
}

/// Strategy for generating EventIds
pub fn event_id_strategy() -> impl Strategy<Value = EventId> {
    any::<u64>().prop_map(EventId)
}

/// Strategy for generating relationship states
pub fn relationship_state_strategy() -> impl Strategy<Value = RelationshipState> {
    prop_oneof![
        Just(RelationshipState::Hostile),
        Just(RelationshipState::Distrustful),
        Just(RelationshipState::Neutral),
        Just(RelationshipState::Friendly),
        Just(RelationshipState::Allied),
    ]
}

/// Strategy for generating character abilities
pub fn ability_strategy() -> impl Strategy<Value = Ability> {
    prop_oneof![
        Just(Ability::TimelinePerception),
        Just(Ability::Precognition),
        Just(Ability::MemoryImmunity),
        Just(Ability::LoopMemory),
        Just(Ability::CausalityHacking),
    ]
}

/// Strategy for generating memory provenance
pub fn memory_provenance_strategy() -> impl Strategy<Value = MemoryProvenance> {
    prop_oneof![
        character_id_strategy().prop_map(|id| MemoryProvenance::Witnessed { character: id }),
        (character_id_strategy(), "[a-z]{5,15}")
            .prop_map(|(id, mechanism)| MemoryProvenance::Traded {
                original_owner: id,
                acquired_via: mechanism,
            }),
        "[A-Z][a-z]{5,15}".prop_map(|forger| MemoryProvenance::Forged { forger }),
    ]
}

/// Strategy for generating causality violations
pub fn causality_violation_strategy() -> impl Strategy<Value = CausalityViolation> {
    prop_oneof![
        "[A-Z][a-z]{5,20}".prop_map(|mechanism| CausalityViolation::EffectBeforeCause {
            mechanism
        }),
        "[A-Z][a-z]{5,20}".prop_map(|mechanism| CausalityViolation::RetroactiveChange {
            mechanism
        }),
        "[A-Z][a-z]{5,20}".prop_map(|mechanism| CausalityViolation::Superposition { mechanism }),
    ]
}

/// Strategy for generating event effects
pub fn event_effect_strategy(
    num_characters: usize,
    num_memories: usize,
) -> impl Strategy<Value = EventEffect> {
    let char_range = 0..num_characters as u64;
    let mem_range = 0..num_memories.max(1) as u64;

    prop_oneof![
        // Character death
        char_range.clone().prop_map(|id| EventEffect::CharacterDeath {
            character: CharacterId(id)
        }),
        // Character resurrection
        (char_range.clone(), "[A-Z][a-z]{5,15}").prop_map(|(id, mechanism)| {
            EventEffect::CharacterResurrection {
                character: CharacterId(id),
                mechanism,
            }
        }),
        // Relationship change
        (char_range.clone(), char_range.clone(), relationship_state_strategy()).prop_map(
            |(id1, id2, state)| EventEffect::RelationshipChange {
                character1: CharacterId(id1),
                character2: CharacterId(id2),
                new_state: state,
            }
        ),
        // Knowledge gained
        (char_range.clone(), "[a-z_]{5,20}").prop_map(|(id, flag)| {
            EventEffect::KnowledgeGained {
                character: CharacterId(id),
                flag,
            }
        }),
        // Memory transfer
        (mem_range, prop::option::of(char_range.clone()), char_range)
            .prop_map(|(mem_id, from, to)| EventEffect::MemoryTransfer {
                memory: MemoryId(mem_id),
                from: from.map(CharacterId),
                to: CharacterId(to),
            }),
    ]
}

/// ## Narrative Action Strategy
///
/// This is the key to generative testing: we define **actions** that can be
/// performed on a Multiverse, then generate random sequences of actions.
///
/// Each action is a valid narrative operation (create character, kill character,
/// trade memory, etc.). By applying random sequences of actions and checking
/// properties after each one, we explore the state space thoroughly.
#[derive(Debug, Clone)]
pub enum NarrativeAction {
    CreateCharacter {
        name: String,
        timeline: TimelineId,
    },
    KillCharacter {
        character: CharacterId,
        timeline: TimelineId,
    },
    ResurrectCharacter {
        character: CharacterId,
        timeline: TimelineId,
        mechanism: String,
    },
    ChangeRelationship {
        char1: CharacterId,
        char2: CharacterId,
        new_state: RelationshipState,
        timeline: TimelineId,
    },
    GrantKnowledge {
        character: CharacterId,
        flag: String,
        timeline: TimelineId,
    },
    TradeMemory {
        memory: MemoryId,
        from: CharacterId,
        to: CharacterId,
        mechanism: String,
    },
    BranchTimeline {
        parent: TimelineId,
    },
    CreateWitnessedMemory {
        event: EventId,
        character: CharacterId,
        timeline: TimelineId,
    },
    ViolateCausality {
        timeline: TimelineId,
        violation_type: CausalityViolation,
    },
    GrantAbility {
        character: CharacterId,
        ability: Ability,
    },
}

/// Strategy for generating narrative actions
pub fn narrative_action_strategy() -> impl Strategy<Value = NarrativeAction> {
    prop_oneof![
        // Create character
        ("[A-Z][a-z]{3,10}", timeline_id_strategy()).prop_map(|(name, timeline)| {
            NarrativeAction::CreateCharacter { name, timeline }
        }),
        // Kill character
        (character_id_strategy(), timeline_id_strategy()).prop_map(|(character, timeline)| {
            NarrativeAction::KillCharacter {
                character,
                timeline,
            }
        }),
        // Resurrect character
        (
            character_id_strategy(),
            timeline_id_strategy(),
            "[A-Z][a-z]{5,15}"
        )
            .prop_map(|(character, timeline, mechanism)| {
                NarrativeAction::ResurrectCharacter {
                    character,
                    timeline,
                    mechanism,
                }
            }),
        // Change relationship
        (
            character_id_strategy(),
            character_id_strategy(),
            relationship_state_strategy(),
            timeline_id_strategy()
        )
            .prop_map(|(char1, char2, new_state, timeline)| {
                NarrativeAction::ChangeRelationship {
                    char1,
                    char2,
                    new_state,
                    timeline,
                }
            }),
        // Grant knowledge
        (character_id_strategy(), "[a-z_]{5,20}", timeline_id_strategy()).prop_map(
            |(character, flag, timeline)| NarrativeAction::GrantKnowledge {
                character,
                flag,
                timeline,
            }
        ),
        // Trade memory
        (
            memory_id_strategy(),
            character_id_strategy(),
            character_id_strategy(),
            "[a-z]{5,15}"
        )
            .prop_map(|(memory, from, to, mechanism)| NarrativeAction::TradeMemory {
                memory,
                from,
                to,
                mechanism,
            }),
        // Branch timeline
        timeline_id_strategy().prop_map(|parent| NarrativeAction::BranchTimeline { parent }),
        // Grant ability
        (character_id_strategy(), ability_strategy()).prop_map(|(character, ability)| {
            NarrativeAction::GrantAbility { character, ability }
        }),
    ]
}

/// ## Applying Actions to Multiverse
///
/// This function takes a narrative action and applies it to a Multiverse,
/// creating appropriate events and updating state.
///
/// This is where the "action interpreter" lives—it translates abstract
/// actions into concrete state changes.
pub fn apply_narrative_action(multiverse: &mut Multiverse, action: &NarrativeAction) {
    match action {
        NarrativeAction::CreateCharacter { name, timeline } => {
            // Only create if timeline exists
            if multiverse.timelines.contains_key(timeline) {
                multiverse.create_character(name.clone(), *timeline);
            }
        }

        NarrativeAction::KillCharacter {
            character,
            timeline,
        } => {
            // Create death event
            if let Some(c) = multiverse.characters.get(character) {
                if c.alive && c.current_timeline == *timeline && multiverse.timelines.contains_key(timeline) {
                    multiverse.record_event(Event {
                        id: EventId(0), // Will be overwritten
                        timeline: *timeline,
                        description: format!("Character {} dies", character),
                        participants: vec![*character].into_iter().collect(),
                        effects: vec![EventEffect::CharacterDeath {
                            character: *character,
                        }],
                        causality_violation: None,
                    });
                }
            }
        }

        NarrativeAction::ResurrectCharacter {
            character,
            timeline,
            mechanism,
        } => {
            if let Some(c) = multiverse.characters.get(character) {
                if c.current_timeline == *timeline && multiverse.timelines.contains_key(timeline) {
                    multiverse.record_event(Event {
                        id: EventId(0),
                        timeline: *timeline,
                        description: format!("Character {} is resurrected", character),
                        participants: vec![*character].into_iter().collect(),
                        effects: vec![EventEffect::CharacterResurrection {
                            character: *character,
                            mechanism: mechanism.clone(),
                        }],
                        causality_violation: None,
                    });
                }
            }
        }

        NarrativeAction::ChangeRelationship {
            char1,
            char2,
            new_state,
            timeline,
        } => {
            if let (Some(c1), Some(c2)) = (multiverse.characters.get(char1), multiverse.characters.get(char2)) {
                if c1.alive && c2.alive && c1.current_timeline == *timeline && c2.current_timeline == *timeline {
                    multiverse.record_event(Event {
                        id: EventId(0),
                        timeline: *timeline,
                        description: format!("Relationship changes between {} and {}", char1, char2),
                        participants: vec![*char1, *char2].into_iter().collect(),
                        effects: vec![EventEffect::RelationshipChange {
                            character1: *char1,
                            character2: *char2,
                            new_state: *new_state,
                        }],
                        causality_violation: None,
                    });
                }
            }
        }

        NarrativeAction::GrantKnowledge {
            character,
            flag,
            timeline,
        } => {
            if let Some(c) = multiverse.characters.get(character) {
                if c.alive && c.current_timeline == *timeline {
                    multiverse.record_event(Event {
                        id: EventId(0),
                        timeline: *timeline,
                        description: format!("Knowledge {} granted to {}", flag, character),
                        participants: vec![*character].into_iter().collect(),
                        effects: vec![EventEffect::KnowledgeGained {
                            character: *character,
                            flag: flag.clone(),
                        }],
                        causality_violation: None,
                    });
                }
            }
        }

        NarrativeAction::TradeMemory {
            memory,
            from,
            to,
            mechanism,
        } => {
            if let (Some(f), Some(t)) = (multiverse.characters.get(from), multiverse.characters.get(to)) {
                if f.alive && t.alive {
                    // Get timeline from recipient
                    let timeline = t.current_timeline;

                    // Only trade if both are in same timeline (for simplicity in random tests)
                    if f.current_timeline == timeline {
                        // Create traded memory if it doesn't exist
                        if !multiverse.memories.contains_key(memory) {
                            let traded_memory = Memory {
                                id: *memory,
                                event: EventId(0), // Dummy event
                                source_timeline: timeline,
                                provenance: MemoryProvenance::Traded {
                                    original_owner: *from,
                                    acquired_via: mechanism.clone(),
                                },
                                fidelity: 0.9,
                            };
                            multiverse.memories.insert(*memory, traded_memory);
                        }

                        multiverse.record_event(Event {
                            id: EventId(0),
                            timeline,
                            description: format!("Memory traded from {} to {}", from, to),
                            participants: vec![*from, *to].into_iter().collect(),
                            effects: vec![EventEffect::MemoryTransfer {
                                memory: *memory,
                                from: Some(*from),
                                to: *to,
                            }],
                            causality_violation: None,
                        });
                    }
                }
            }
        }

        NarrativeAction::BranchTimeline { parent } => {
            if let Some(parent_timeline) = multiverse.timelines.get(parent) {
                if !parent_timeline.events.is_empty() {
                    let divergence_event = *parent_timeline.events.last().unwrap();
                    multiverse.create_timeline_branch(*parent, divergence_event);
                }
            }
        }

        NarrativeAction::CreateWitnessedMemory {
            event,
            character,
            timeline,
        } => {
            if let Some(c) = multiverse.characters.get(character) {
                if c.alive && c.current_timeline == *timeline && multiverse.timelines.contains_key(timeline) {
                    let memory_id = multiverse.create_witnessed_memory(*event, *timeline, *character);
                    if let Some(c_mut) = multiverse.characters.get_mut(character) {
                        c_mut.memories.insert(memory_id);
                    }
                }
            }
        }

        NarrativeAction::ViolateCausality {
            timeline,
            violation_type,
        } => {
            if let Some(timeline_data) = multiverse.timelines.get_mut(timeline) {
                timeline_data.causality_stable = false;

                multiverse.record_event(Event {
                    id: EventId(0),
                    timeline: *timeline,
                    description: "Causality violation occurs".to_string(),
                    participants: std::collections::HashSet::new(),
                    effects: vec![],
                    causality_violation: Some(violation_type.clone()),
                });
            }
        }

        NarrativeAction::GrantAbility { character, ability } => {
            if let Some(c) = multiverse.characters.get_mut(character) {
                if c.alive {
                    c.abilities.insert(ability.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn test_action_generation(action in narrative_action_strategy()) {
            // Just verify we can generate actions without panicking
            let _ = format!("{:?}", action);
        }

        #[test]
        fn test_action_sequence(actions in prop::collection::vec(narrative_action_strategy(), 1..20)) {
            let mut multiverse = Multiverse::new();

            // Apply all actions
            for action in &actions {
                apply_narrative_action(&mut multiverse, action);
            }

            // Multiverse should still be valid
            assert!(multiverse.timelines.len() > 0);
        }
    }
}
