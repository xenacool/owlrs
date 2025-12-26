//! # Property Tests: Validating Narrative Coherence
//!
//! This module contains property-based tests that validate narrative consistency
//! across thousands of randomly-generated story sequences.
//!
//! ## Philosophy
//!
//! Traditional narrative testing checks specific scenarios:
//! "If the player kills Character X, then Y happens."
//!
//! Property-based testing checks **invariants across all possible scenarios**:
//! "Dead characters NEVER speak (unless resurrected via explicit mechanism)."
//!
//! By generating thousands of random action sequences and checking invariants,
//! we catch edge cases that manual testing would miss.
//!
//! ## Core Properties
//!
//! Based on the conversation between Armstrong, Lamport, Kurvitz, and Compton,
//! we implement these narrative properties:
//!
//! 1. **Memory Consistency**: Characters only remember events they witnessed OR acquired via memory trade
//! 2. **Timeline Isolation**: Characters can't reference events from timelines they haven't perceived
//! 3. **Causality Justification**: Causality violations require explicit in-world mechanisms
//! 4. **Relationship Persistence**: Relationships stay consistent within a timeline
//! 5. **Death Finality**: Dead characters can't act (unless resurrected)
//! 6. **Knowledge Propagation**: Knowledge flags are set correctly after events

use crate::narrative_core::*;

/// ## Property 1: Memory Consistency
///
/// **Invariant**: A character can only have a memory of an event if:
/// - They witnessed the event (were present), OR
/// - They acquired the memory via explicit trade/installation
///
/// This prevents the common bug where characters mysteriously "know" things
/// they shouldn't.
pub fn prop_memory_consistency(multiverse: &Multiverse) -> Result<(), String> {
    for (char_id, character) in &multiverse.characters {
        for memory_id in &character.memories {
            let memory = multiverse
                .memories
                .get(memory_id)
                .ok_or_else(|| format!("Memory {} not found in multiverse", memory_id))?;

            // Check if memory is justified
            match &memory.provenance {
                MemoryProvenance::Witnessed { character: witness } => {
                    // Verify the character was actually present at the event
                    if let Some(event) = multiverse.events.get(&memory.event) {
                        if !event.participants.contains(witness) {
                            return Err(format!(
                                "{} has witnessed memory of event {}, but was not present",
                                char_id, memory.event.0
                            ));
                        }
                    }
                }
                MemoryProvenance::Traded { .. } => {
                    // Memory trades are justified by the trade mechanism
                    // (validated elsewhere)
                }
                MemoryProvenance::Forged { forger } => {
                    // Forged memories must have a justification
                    if forger.is_empty() {
                        return Err(format!(
                            "{} has forged memory {} with no forger specified",
                            char_id, memory_id.0
                        ));
                    }
                }
                MemoryProvenance::Compound { sources } => {
                    // Verify all source memories exist
                    for source_id in sources {
                        if !multiverse.memories.contains_key(source_id) {
                            return Err(format!(
                                "Compound memory {} references non-existent source {}",
                                memory_id.0, source_id.0
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// ## Property 2: Timeline Perception
///
/// **Invariant**: A character can only reference/perceive events from timelines if:
/// - They are currently in that timeline, OR
/// - They have the TimelinePerception ability
///
/// This prevents timeline-specific knowledge from bleeding across branches.
pub fn prop_timeline_perception(multiverse: &Multiverse) -> Result<(), String> {
    for (char_id, character) in &multiverse.characters {
        // Check all memories this character has
        for memory_id in &character.memories {
            let memory = multiverse
                .memories
                .get(memory_id)
                .ok_or_else(|| format!("Memory {} not found", memory_id))?;

            // If memory is from a different timeline, character must be able to perceive it
            if memory.source_timeline != character.current_timeline
                && !character
                    .abilities
                    .contains(&Ability::TimelinePerception)
            {
                return Err(format!(
                    "{} ({}) has memory from {} but is in {} without TimelinePerception ability",
                    character.name, char_id, memory.source_timeline, character.current_timeline
                ));
            }
        }
    }
    Ok(())
}

/// ## Property 3: Causality Violation Justification
///
/// **Invariant**: Events can only violate causality if they have an explicit
/// in-universe mechanism (Gates, time weapons, etc.)
///
/// This prevents arbitrary causality breaks that confuse players.
pub fn prop_causality_justification(multiverse: &Multiverse) -> Result<(), String> {
    for event in multiverse.events.values() {
        if let Some(violation) = &event.causality_violation {
            // Verify the violation has a mechanism
            let mechanism = match violation {
                CausalityViolation::EffectBeforeCause { mechanism } => mechanism,
                CausalityViolation::RetroactiveChange { mechanism } => mechanism,
                CausalityViolation::Superposition { mechanism } => mechanism,
            };

            if mechanism.is_empty() {
                return Err(format!(
                    "Event {} violates causality without specified mechanism",
                    event.id.0
                ));
            }

            // Timeline should be marked as causality-unstable
            if let Some(timeline) = multiverse.timelines.get(&event.timeline) {
                if timeline.causality_stable {
                    return Err(format!(
                        "Event {} violates causality but timeline {} is marked stable",
                        event.id.0, timeline.id
                    ));
                }
            }
        }
    }
    Ok(())
}

/// ## Property 4: Relationship Consistency
///
/// **Invariant**: Within a single timeline, character relationships must remain
/// consistent—they can only change via explicit relationship-change events.
///
/// This prevents relationships from randomly fluctuating.
pub fn prop_relationship_consistency(multiverse: &Multiverse) -> Result<(), String> {
    use std::collections::HashMap;

    // For each timeline, verify relationships are justified by events
    for timeline in multiverse.timelines.values() {
        let mut relationship_history: HashMap<(CharacterId, CharacterId), Vec<RelationshipState>> =
            HashMap::new();

        // Walk through events in order
        for event_id in &timeline.events {
            if let Some(event) = multiverse.events.get(event_id) {
                for effect in &event.effects {
                    if let EventEffect::RelationshipChange {
                        character1,
                        character2,
                        new_state,
                    } = effect
                    {
                        relationship_history
                            .entry((*character1, *character2))
                            .or_insert_with(Vec::new)
                            .push(*new_state);
                    }
                }
            }
        }

        // Now verify current relationships match the last recorded change
        for char_id in &timeline.characters {
            if let Some(character) = multiverse.characters.get(char_id) {
                for (other_id, current_state) in &character.relationships {
                    let key = (*char_id, *other_id);
                    if let Some(history) = relationship_history.get(&key) {
                        if let Some(last_state) = history.last() {
                            if last_state != current_state {
                                return Err(format!(
                                    "Relationship between {} and {} is {:?} but last event set it to {:?}",
                                    char_id, other_id, current_state, last_state
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// ## Property 5: Death Finality
///
/// **Invariant**: Dead characters cannot participate in events unless
/// they've been explicitly resurrected via a resurrection mechanism.
pub fn prop_death_finality(multiverse: &Multiverse) -> Result<(), String> {
    use std::collections::HashMap;

    // Track death/resurrection events in each timeline
    let mut character_alive_state: HashMap<TimelineId, HashMap<CharacterId, bool>> =
        HashMap::new();

    // Sort timelines by ID to ensure we process parents before children
    // (TimelineId is sequential)
    let mut timelines: Vec<_> = multiverse.timelines.values().collect();
    timelines.sort_by_key(|t| t.id.0);

    for timeline in timelines {
        let mut alive_in_timeline = HashMap::new();

        // If this is a branched timeline, inherit the state from parent
        if let Some(parent_id) = timeline.parent {
            if let Some(parent_state) = character_alive_state.get(&parent_id) {
                alive_in_timeline = parent_state.clone();
            }
        } else {
            // Root timeline: all characters start alive
            for char_id in &timeline.characters {
                alive_in_timeline.insert(*char_id, true);
            }
        }

        // Process events in order
        for event_id in &timeline.events {
            if let Some(event) = multiverse.events.get(event_id) {
                // Check participants are all alive
                for participant in &event.participants {
                    if !alive_in_timeline.get(participant).copied().unwrap_or(false) {
                        // Check if this event is a resurrection that includes them
                        let is_resurrection = event.effects.iter().any(|effect| {
                            matches!(effect, EventEffect::CharacterResurrection { character, .. } if character == participant)
                        });

                        if !is_resurrection {
                            let name = multiverse.characters.get(participant).map(|c| c.name.as_str()).unwrap_or("Unknown");
                            return Err(format!(
                                "Dead character {} ({}) participates in event {} without resurrection",
                                participant, name, event.id.0
                            ));
                        }
                    }
                }

                // Apply death/resurrection effects
                for effect in &event.effects {
                    match effect {
                        EventEffect::CharacterDeath { character } => {
                            alive_in_timeline.insert(*character, false);
                        }
                        EventEffect::CharacterResurrection { character, mechanism } => {
                            if mechanism.is_empty() {
                                return Err(format!(
                                    "Character {} resurrected without mechanism",
                                    character
                                ));
                            }
                            alive_in_timeline.insert(*character, true);
                        }
                        _ => {}
                    }
                }
            }
        }

        character_alive_state.insert(timeline.id, alive_in_timeline);
    }

    // Verify character alive status matches their timeline's state
    for character in multiverse.characters.values() {
        if let Some(alive_in_timeline) = character_alive_state.get(&character.current_timeline) {
            let expected_alive = alive_in_timeline
                .get(&character.id)
                .copied()
                .unwrap_or(true);

            if character.alive != expected_alive {
                return Err(format!(
                    "Character {} ({}) alive status is {} but should be {} based on events in timeline {}",
                    character.id, character.name, character.alive, expected_alive, character.current_timeline
                ));
            }
        }
    }

    Ok(())
}

/// ## Property 6: Knowledge Flag Propagation
///
/// **Invariant**: If a character has a knowledge flag set, there must be
/// an event in their timeline that granted that knowledge.
pub fn prop_knowledge_flags(multiverse: &Multiverse) -> Result<(), String> {
    use std::collections::{HashMap, HashSet};

    // Track knowledge granted in each timeline
    let mut knowledge_granted: HashMap<TimelineId, HashMap<CharacterId, HashSet<String>>> =
        HashMap::new();

    for timeline in multiverse.timelines.values() {
        let mut granted = HashMap::new();

        for event_id in &timeline.events {
            if let Some(event) = multiverse.events.get(event_id) {
                for effect in &event.effects {
                    if let EventEffect::KnowledgeGained { character, flag } = effect {
                        granted
                            .entry(*character)
                            .or_insert_with(HashSet::new)
                            .insert(flag.clone());
                    }
                }
            }
        }

        knowledge_granted.insert(timeline.id, granted);
    }

    // Verify each character's knowledge flags are justified
    for character in multiverse.characters.values() {
        if let Some(granted) = knowledge_granted.get(&character.current_timeline) {
            if let Some(char_knowledge) = granted.get(&character.id) {
                for flag in &character.knowledge_flags {
                    if !char_knowledge.contains(flag) {
                        return Err(format!(
                            "Character {} has knowledge flag '{}' but no event granted it",
                            character.id, flag
                        ));
                    }
                }
            } else if !character.knowledge_flags.is_empty() {
                return Err(format!(
                    "Character {} has knowledge flags but no events granted any",
                    character.id
                ));
            }
        }
    }

    Ok(())
}

/// ## Combined Property Validator
///
/// Runs all property checks on a multiverse state.
/// Returns Ok(()) if all properties hold, or Err with details of the first violation.
pub fn validate_all_properties(multiverse: &Multiverse) -> Result<(), String> {
    prop_memory_consistency(multiverse)?;
    prop_timeline_perception(multiverse)?;
    prop_causality_justification(multiverse)?;
    prop_relationship_consistency(multiverse)?;
    prop_death_finality(multiverse)?;
    prop_knowledge_flags(multiverse)?;
    prop_emotional_state_validity(multiverse)?;
    Ok(())
}

/// Invariant: Emotional PAD values must always be between -1.0 and 1.0.
pub fn prop_emotional_state_validity(multiverse: &Multiverse) -> Result<(), String> {
    for character in multiverse.characters.values() {
        let pad = character.emotional_state.get_pad();
        for (i, val) in pad.iter().enumerate() {
            if *val < -1.0 || *val > 1.0 {
                return Err(format!(
                    "Character {} has invalid PAD value at index {}: {}",
                    character.name, i, val
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_memory_consistency_witnessed() {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;
        let char1 = multiverse.create_character("Alice".to_string(), timeline);

        // Create event with Alice as participant
        let event_id = multiverse.record_event(Event {
            id: EventId(0),
            timeline,
            description: "Alice sees something".to_string(),
            participants: HashSet::from([char1]),
            effects: vec![],
            causality_violation: None,
        });

        // Create witnessed memory
        let memory_id = multiverse.create_witnessed_memory(event_id, timeline, char1);

        // Add memory to character
        if let Some(character) = multiverse.characters.get_mut(&char1) {
            character.memories.insert(memory_id);
        }

        // Should pass memory consistency check
        assert!(prop_memory_consistency(&multiverse).is_ok());
    }

    #[test]
    fn test_memory_consistency_violation() {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;
        let char1 = multiverse.create_character("Alice".to_string(), timeline);
        let char2 = multiverse.create_character("Bob".to_string(), timeline);

        // Event where only Alice is present
        let event_id = multiverse.record_event(Event {
            id: EventId(0),
            timeline,
            description: "Alice-only event".to_string(),
            participants: HashSet::from([char1]),
            effects: vec![],
            causality_violation: None,
        });

        // Create witnessed memory but claim Bob witnessed it (violation!)
        let memory_id = multiverse.create_witnessed_memory(event_id, timeline, char2);

        // Add memory to Bob
        if let Some(character) = multiverse.characters.get_mut(&char2) {
            character.memories.insert(memory_id);
        }

        // Should FAIL memory consistency check
        assert!(prop_memory_consistency(&multiverse).is_err());
    }

    #[test]
    fn test_death_finality() {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;
        let char1 = multiverse.create_character("Victim".to_string(), timeline);

        // Event that kills character
        multiverse.record_event(Event {
            id: EventId(0),
            timeline,
            description: "Character dies".to_string(),
            participants: HashSet::from([char1]),
            effects: vec![EventEffect::CharacterDeath { character: char1 }],
            causality_violation: None,
        });

        // Try to have dead character participate in another event (violation!)
        multiverse.record_event(Event {
            id: EventId(1),
            timeline,
            description: "Dead character speaks".to_string(),
            participants: HashSet::from([char1]),
            effects: vec![],
            causality_violation: None,
        });

        // Should FAIL death finality check
        assert!(prop_death_finality(&multiverse).is_err());
    }
}
