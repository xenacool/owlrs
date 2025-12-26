//! # Narrative Core: A Literate Implementation of Multi-Timeline Story Systems
//!
//! This module implements the core data structures and logic for managing a narrative
//! system with multiple timelines, memory trading, causality violations, and character
//! relationship tracking—all designed to be validated through property-based testing.
//!
//! ## Design Philosophy
//!
//! Traditional branching narratives explode in state-space complexity. With 13 protagonists,
//! multiple timelines, memory trading, and reality manipulation, naïve implementations
//! create thousands of invisible inconsistencies:
//!
//! - Characters reference events they never witnessed
//! - Dead characters speak without resurrection mechanics
//! - Timeline-specific knowledge bleeds across branches
//! - Causality violations occur without in-universe justification
//!
//! We solve this by encoding **narrative invariants as executable properties** that are
//! continuously validated against thousands of randomly-generated story sequences.
//!
//! ## Core Concepts
//!
//! ### Timelines
//!
//! A timeline is a causally-consistent sequence of events. In our system, timelines can:
//! - **Branch**: A choice creates a new timeline diverging from a parent
//! - **Merge**: (Rare) Two timelines converge under specific conditions
//! - **Isolate**: Become unreachable from other timelines
//!
//! Each timeline has a unique identifier and tracks its divergence point from parents.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// A unique identifier for a timeline.
///
/// Timelines are the fundamental unit of branching narrative. Each represents
/// a causally-consistent sequence of events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimelineId(pub u64);

impl fmt::Display for TimelineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timeline#{}", self.0)
    }
}

/// A unique identifier for a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharacterId(pub u64);

impl fmt::Display for CharacterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Char#{}", self.0)
    }
}

/// A unique identifier for a memory fragment.
///
/// Memories can be extracted, traded, forged, and installed. Each memory
/// has a provenance that tracks its origin and any modifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub u64);

impl fmt::Display for MemoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Memory#{}", self.0)
    }
}

/// A unique identifier for an event in the narrative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub u64);

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event#{}", self.0)
    }
}

/// ## Memory System
///
/// Memories are first-class entities in this narrative system. They can be:
///
/// 1. **Extracted** from a character's mind
/// 2. **Traded** between characters or stored in memory banks
/// 3. **Forged** (fabricated from whole cloth)
/// 4. **Compound** (blended from multiple perspectives)
/// 5. **Installed** into a character's mind
///
/// Each memory tracks its **provenance** to enable validation properties like:
/// - "Characters can only remember events they witnessed OR acquired via memory trade"
/// - "Forged memories must have an in-universe justification (Memory Cartel, etc.)"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: MemoryId,
    pub event: EventId,
    /// The timeline this memory is "from" (may differ from character's current timeline)
    pub source_timeline: TimelineId,
    /// How this memory came to exist
    pub provenance: MemoryProvenance,
    /// Fidelity: 1.0 = perfect recall, 0.0 = completely degraded
    pub fidelity: f32,
}

/// Tracks how a memory was created.
///
/// This is crucial for property testing—we can verify that characters only have
/// memories that are justified by game events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryProvenance {
    /// Directly witnessed by the original character
    Witnessed { character: CharacterId },
    /// Extracted and traded from another character
    Traded {
        original_owner: CharacterId,
        acquired_via: String, // "Memory Market", "Gate payment", etc.
    },
    /// Fabricated by a faction or entity
    Forged { forger: String },
    /// Blended from multiple source memories
    Compound { sources: Vec<MemoryId> },
}

/// ## Character State
///
/// Each character exists in a specific timeline and carries:
/// - **Memories**: What they remember (from any timeline)
/// - **Knowledge flags**: Abstract facts they know
/// - **Relationship state**: How they feel about other characters
/// - **Alive status**: Dead characters can't act (unless resurrected)
/// - **Abilities**: Special powers like timeline-perception, precognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    /// The timeline this character currently inhabits
    pub current_timeline: TimelineId,
    /// Their native timeline (where they originated)
    pub native_timeline: TimelineId,
    /// All memories this character possesses
    pub memories: HashSet<MemoryId>,
    /// Abstract knowledge flags ("kim_betrayal_acknowledged", etc.)
    pub knowledge_flags: HashSet<String>,
    /// Is this character alive in their current timeline?
    pub alive: bool,
    /// Special abilities that affect property validation
    pub abilities: HashSet<Ability>,
    /// Relationship values with other characters (in current timeline)
    pub relationships: HashMap<CharacterId, RelationshipState>,
    /// Emotional state and goals (Gamygdala/PAD system)
    pub emotional_state: crate::emotional_system::EmotionalState,
}

/// Special abilities that grant exceptions to normal narrative rules.
///
/// For example, a character with `TimelinePerception` can reference events
/// from alternate timelines without violating the consistency property.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ability {
    /// Can perceive multiple timelines simultaneously
    TimelinePerception,
    /// Can see possible futures (may be contradictory)
    Precognition,
    /// Immune to memory manipulation
    MemoryImmunity,
    /// Remembers across time loops
    LoopMemory,
    /// Can manipulate causality directly
    CausalityHacking,
}

/// Relationship states between characters.
///
/// These must remain consistent within a timeline but can differ across branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelationshipState {
    Hostile = -2,
    Distrustful = -1,
    Neutral = 0,
    Friendly = 1,
    Allied = 2,
}

/// ## Timeline Structure
///
/// A timeline is a branching point in the narrative. It tracks:
/// - When it diverged from its parent
/// - What characters exist in this timeline
/// - What events have occurred
/// - Whether causality is stable or violated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub id: TimelineId,
    /// The timeline this branched from (None for the original timeline)
    pub parent: Option<TimelineId>,
    /// The event that caused this timeline to diverge
    pub divergence_event: Option<EventId>,
    /// All events that have occurred in this timeline (in order)
    pub events: Vec<EventId>,
    /// Characters that exist in this timeline
    pub characters: HashSet<CharacterId>,
    /// Whether causality is coherent in this timeline
    pub causality_stable: bool,
}

/// ## Events
///
/// Events are the atomic units of narrative progression. Each event:
/// - Occurs in a specific timeline
/// - Involves one or more characters
/// - May affect character state, relationships, or memories
/// - May have causality-violating properties (precedes its cause, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub timeline: TimelineId,
    pub description: String,
    /// Characters present for this event
    pub participants: HashSet<CharacterId>,
    /// Effects of this event on game state
    pub effects: Vec<EventEffect>,
    /// Does this event violate normal causality?
    pub causality_violation: Option<CausalityViolation>,
}

/// Effects that events can have on the game state.
///
/// These are tracked explicitly so property tests can verify state changes
/// are properly propagated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventEffect {
    /// Character dies in this timeline
    CharacterDeath { character: CharacterId },
    /// Character is resurrected
    CharacterResurrection {
        character: CharacterId,
        mechanism: String,
    },
    /// Relationship change between two characters
    RelationshipChange {
        character1: CharacterId,
        character2: CharacterId,
        new_state: RelationshipState,
    },
    /// Knowledge flag is set
    KnowledgeGained {
        character: CharacterId,
        flag: String,
    },
    /// Memory is traded or installed
    MemoryTransfer {
        memory: MemoryId,
        from: Option<CharacterId>,
        to: CharacterId,
    },
    /// Timeline branches at this point
    TimelineBranch { new_timeline: TimelineId },
    /// An emotional appraisal event (Gamygdala belief)
    AppraisalTrigger {
        character: CharacterId,
        belief: crate::emotional_system::Belief,
    },
    /// Add a goal to a character
    AddGoal {
        character: CharacterId,
        goal: crate::emotional_system::Goal,
    },
}

/// Types of causality violations that can occur.
///
/// These must have in-universe justifications (Gates, time weapons, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CausalityViolation {
    /// Effect precedes cause (e.g., Riven's time-gun)
    EffectBeforeCause {
        mechanism: String, // "Time-weapon", "Gate manipulation"
    },
    /// Event retroactively changes the past
    RetroactiveChange { mechanism: String },
    /// Event exists in quantum superposition
    Superposition { mechanism: String },
}

/// ## The Multiverse
///
/// The top-level container for all narrative state. Tracks:
/// - All timelines
/// - All characters (across all timelines)
/// - All memories
/// - All events
///
/// This is the structure that property tests will generate and validate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiverse {
    pub timelines: HashMap<TimelineId, Timeline>,
    pub characters: HashMap<CharacterId, Character>,
    pub memories: HashMap<MemoryId, Memory>,
    pub events: HashMap<EventId, Event>,
    /// The "canonical" timeline (usually Timeline#0)
    pub root_timeline: TimelineId,
    /// Counter for generating unique IDs
    next_timeline_id: u64,
    next_character_id: u64,
    next_memory_id: u64,
    next_event_id: u64,
}

impl Multiverse {
    /// Creates a new multiverse with a single root timeline.
    pub fn new() -> Self {
        let root_timeline = TimelineId(0);
        let mut timelines = HashMap::new();
        timelines.insert(
            root_timeline,
            Timeline {
                id: root_timeline,
                parent: None,
                divergence_event: None,
                events: Vec::new(),
                characters: HashSet::new(),
                causality_stable: true,
            },
        );

        Multiverse {
            timelines,
            characters: HashMap::new(),
            memories: HashMap::new(),
            events: HashMap::new(),
            root_timeline,
            next_timeline_id: 1,
            next_character_id: 0,
            next_memory_id: 0,
            next_event_id: 0,
        }
    }

    /// Creates a new character in the specified timeline.
    pub fn create_character(&mut self, name: String, timeline: TimelineId) -> CharacterId {
        let id = CharacterId(self.next_character_id);
        self.next_character_id += 1;

        let character = Character {
            id,
            name,
            current_timeline: timeline,
            native_timeline: timeline,
            memories: HashSet::new(),
            knowledge_flags: HashSet::new(),
            alive: true,
            abilities: HashSet::new(),
            relationships: HashMap::new(),
            emotional_state: crate::emotional_system::EmotionalState::new(),
        };

        self.characters.insert(id, character);

        if let Some(timeline_data) = self.timelines.get_mut(&timeline) {
            timeline_data.characters.insert(id);
        }

        id
    }

    /// Creates a new timeline branching from a parent.
    pub fn create_timeline_branch(
        &mut self,
        parent: TimelineId,
        divergence_event: EventId,
    ) -> TimelineId {
        let id = TimelineId(self.next_timeline_id);
        self.next_timeline_id += 1;

        // Copy characters from parent timeline
        let parent_characters = self
            .timelines
            .get(&parent)
            .map(|t| t.characters.clone())
            .unwrap_or_default();

        let timeline = Timeline {
            id,
            parent: Some(parent),
            divergence_event: Some(divergence_event),
            events: Vec::new(),
            characters: parent_characters,
            causality_stable: true,
        };

        self.timelines.insert(id, timeline);
        id
    }

    /// Creates a memory from a witnessed event.
    pub fn create_witnessed_memory(
        &mut self,
        event: EventId,
        timeline: TimelineId,
        character: CharacterId,
    ) -> MemoryId {
        let id = MemoryId(self.next_memory_id);
        self.next_memory_id += 1;

        let memory = Memory {
            id,
            event,
            source_timeline: timeline,
            provenance: MemoryProvenance::Witnessed { character },
            fidelity: 1.0,
        };

        self.memories.insert(id, memory);
        id
    }

    /// Records a new event in the timeline.
    pub fn record_event(&mut self, event: Event) -> EventId {
        let id = EventId(self.next_event_id);
        self.next_event_id += 1;

        let mut event = event;
        event.id = id;

        // Add to timeline's event list
        if let Some(timeline) = self.timelines.get_mut(&event.timeline) {
            timeline.events.push(id);
        }

        // Apply event effects
        self.apply_event_effects(&event);

        self.events.insert(id, event);
        id
    }

    /// Applies the effects of an event to the multiverse state.
    fn apply_event_effects(&mut self, event: &Event) {
        for effect in &event.effects {
            match effect {
                EventEffect::CharacterDeath { character } => {
                    if let Some(c) = self.characters.get_mut(character) {
                        c.alive = false;
                    }
                }
                EventEffect::CharacterResurrection { character, .. } => {
                    if let Some(c) = self.characters.get_mut(character) {
                        c.alive = true;
                    }
                }
                EventEffect::RelationshipChange {
                    character1,
                    character2,
                    new_state,
                } => {
                    if let Some(c1) = self.characters.get_mut(character1) {
                        c1.relationships.insert(*character2, *new_state);
                    }
                    if let Some(c2) = self.characters.get_mut(character2) {
                        c2.relationships.insert(*character1, *new_state);
                    }
                }
                EventEffect::KnowledgeGained { character, flag } => {
                    if let Some(c) = self.characters.get_mut(character) {
                        c.knowledge_flags.insert(flag.clone());
                    }
                }
                EventEffect::MemoryTransfer { memory, to, .. } => {
                    if let Some(c) = self.characters.get_mut(to) {
                        c.memories.insert(*memory);
                    }
                }
                EventEffect::TimelineBranch { new_timeline } => {
                    // Timeline branching is handled separately
                    let _ = new_timeline;
                }
                EventEffect::AppraisalTrigger { character, belief } => {
                    if let Some(c) = self.characters.get_mut(character) {
                        c.emotional_state.appraise(belief);
                    }
                }
                EventEffect::AddGoal { character, goal } => {
                    if let Some(c) = self.characters.get_mut(character) {
                        c.emotional_state.add_goal(goal.clone());
                    }
                }
            }
        }
    }

    /// Checks if a character can perceive events from a specific timeline.
    ///
    /// Returns true if:
    /// - The character is in that timeline, OR
    /// - The character has TimelinePerception ability
    pub fn can_perceive_timeline(&self, character: CharacterId, timeline: TimelineId) -> bool {
        if let Some(c) = self.characters.get(&character) {
            c.current_timeline == timeline || c.abilities.contains(&Ability::TimelinePerception)
        } else {
            false
        }
    }

    /// Checks if a character has a memory of a specific event.
    pub fn has_memory_of_event(&self, character: CharacterId, event: EventId) -> bool {
        if let Some(c) = self.characters.get(&character) {
            c.memories.iter().any(|memory_id| {
                self.memories
                    .get(memory_id)
                    .map(|m| m.event == event)
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    /// Decays emotions for all characters in all timelines.
    pub fn decay_emotions(&mut self, decay_factor: f64) {
        for character in self.characters.values_mut() {
            character.emotional_state.decay(decay_factor);
        }
    }
}

impl Default for Multiverse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_multiverse() {
        let multiverse = Multiverse::new();
        assert_eq!(multiverse.timelines.len(), 1);
        assert_eq!(multiverse.characters.len(), 0);
    }

    #[test]
    fn test_create_character() {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;
        let char_id = multiverse.create_character("Vera".to_string(), timeline);

        assert_eq!(multiverse.characters.len(), 1);
        assert!(multiverse.characters.contains_key(&char_id));

        let character = &multiverse.characters[&char_id];
        assert_eq!(character.name, "Vera");
        assert_eq!(character.current_timeline, timeline);
        assert!(character.alive);
    }

    #[test]
    fn test_timeline_branching() {
        let mut multiverse = Multiverse::new();
        let root = multiverse.root_timeline;

        // Create a character in root timeline
        let char_id = multiverse.create_character("Khelis".to_string(), root);

        // Create an event that causes a branch
        let event_id = multiverse.record_event(Event {
            id: EventId(0), // Will be overwritten
            timeline: root,
            description: "Player makes a choice".to_string(),
            participants: HashSet::from([char_id]),
            effects: vec![],
            causality_violation: None,
        });

        // Branch the timeline
        let new_timeline = multiverse.create_timeline_branch(root, event_id);

        assert_eq!(multiverse.timelines.len(), 2);
        let branch = &multiverse.timelines[&new_timeline];
        assert_eq!(branch.parent, Some(root));
        assert!(branch.characters.contains(&char_id));
    }
}
