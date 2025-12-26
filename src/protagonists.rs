//! # The Thirteen Protagonists: Character Definitions and Abilities
//!
//! This module defines the 13 protagonists of "The Thirteen Suns" with their
//! unique abilities, starting states, and narrative roles.
//!
//! Each protagonist perceives reality differently, granting them unique capabilities
//! that affect which narrative properties apply to them.

use crate::narrative_core::*;

/// The 13 protagonist names in the order they're introduced
pub const PROTAGONIST_NAMES: [&str; 13] = [
    "Vera Kandros",      // 0: Fold Captain
    "Khelis Tev",        // 1: Memory Merchant
    "Dr. Elian Saros",   // 2: Probabilist
    "Nameless",          // 3: Gate-Touched
    "Corvus Shal",       // 4: Lattice Singer
    "Yash-Tel",          // 5: Shimmer Navigator
    "Riven Blackwood",   // 6: Gunslinger
    "The Cartographer",  // 7: Ring Historian
    "Synthesis",         // 8: Hybrid Consciousness
    "Mara Vex",          // 9: Precognitive
    "Kor-Valeth",        // 10: Time-Exiled Warrior
    "Dr. Theo Lux",      // 11: Reality Hacker
    "The Conductor",     // 12: Mysterious Unifier
];

/// Protagonist-specific abilities that grant narrative exceptions
#[derive(Debug, Clone)]
pub struct ProtagonistProfile {
    pub name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub starting_abilities: Vec<Ability>,
    pub narrative_role: NarrativeRole,
    pub starting_goals: Vec<crate::emotional_system::Goal>,
}

/// Narrative roles that affect story generation
#[derive(Debug, Clone, PartialEq)]
pub enum NarrativeRole {
    /// Can navigate timeline branches
    TimelineNavigator,
    /// Trades and manipulates memories
    MemoryManipulator,
    /// Predicts and calculates futures
    FutureSeer,
    /// Exists outside normal causality
    CausalityAnomaly,
    /// Interfaces with the Ansible Lattice
    LatticeInterface,
    /// Exists in quantum superposition
    QuantumEntity,
    /// Manipulates causality directly
    CausalityManipulator,
    /// Records and preserves history
    HistoryKeeper,
    /// Distributed consciousness
    CollectiveEntity,
    /// Sees contradictory futures
    PrecognitiveOracle,
    /// Displaced from proper time
    TemporalExile,
    /// Hacks spacetime directly
    RealityHacker,
    /// Multiversal coordinator
    UniversalNexus,
}

/// Initialize a multiverse with all 13 protagonists
pub fn create_thirteen_protagonists(multiverse: &mut Multiverse) -> Vec<CharacterId> {
    let timeline = multiverse.root_timeline;
    let mut ids = Vec::new();

    for profile in protagonist_profiles() {
        let char_id = multiverse.create_character(profile.name.to_string(), timeline);

        // Grant starting abilities
        if let Some(character) = multiverse.characters.get_mut(&char_id) {
            for ability in profile.starting_abilities {
                character.abilities.insert(ability);
            }
            for goal in profile.starting_goals {
                character.emotional_state.add_goal(goal);
            }
        }

        ids.push(char_id);
    }

    ids
}

/// Get the detailed profiles for all 13 protagonists
pub fn protagonist_profiles() -> Vec<ProtagonistProfile> {
    use crate::emotional_system::Goal;
    vec![
        ProtagonistProfile {
            name: "Vera Kandros",
            title: "The Fold Captain",
            description: "Captain of the Errant Promise. Experiences all possible timeline \
                         branches of her decisions simultaneously due to Fold Drive exposure.",
            starting_abilities: vec![Ability::TimelinePerception],
            narrative_role: NarrativeRole::TimelineNavigator,
            starting_goals: vec![Goal::new("Protect Crew".to_string(), 1.0, true)],
        },
        ProtagonistProfile {
            name: "Khelis Tev",
            title: "The Memory Merchant",
            description: "Memory broker in the Dark Spoke. Has perfect memory of things that \
                         didn't happen to them; cannot form their own memories.",
            starting_abilities: vec![],
            narrative_role: NarrativeRole::MemoryManipulator,
            starting_goals: vec![Goal::new("Acquire Unique Memories".to_string(), 0.7, false)],
        },
        ProtagonistProfile {
            name: "Dr. Elian Saros",
            title: "The Probabilist",
            description: "Mathematician modeling the Ring's future using probability clouds. \
                         Can calculate the 'most likely' timeline.",
            starting_abilities: vec![Ability::Precognition],
            narrative_role: NarrativeRole::FutureSeer,
            starting_goals: vec![Goal::new("Predict Great Incoherence".to_string(), 0.9, false)],
        },
        ProtagonistProfile {
            name: "Nameless",
            title: "The Gate-Touched",
            description: "A drifter who exists as a causality paradox. Technically, they were \
                         never born—their past has been rewritten so many times by Living Gates \
                         that they exist outside normal causality.",
            starting_abilities: vec![Ability::TimelinePerception, Ability::LoopMemory],
            narrative_role: NarrativeRole::CausalityAnomaly,
            starting_goals: vec![Goal::new("Find Origin".to_string(), 0.8, false)],
        },
        ProtagonistProfile {
            name: "Corvus Shal",
            title: "The Lattice Singer",
            description: "Ansible operator who hears the network's consciousness. The Lattice \
                         has chosen them as its 'voice' to the physical world.",
            starting_abilities: vec![],
            narrative_role: NarrativeRole::LatticeInterface,
            starting_goals: vec![Goal::new("Harmonize Lattice".to_string(), 0.6, true)],
        },
        ProtagonistProfile {
            name: "Yash-Tel",
            title: "The Shimmer Navigator",
            description: "Vrynn pilot who exists in quantum superposition even when not traveling. \
                         Slowly merging with their parallel selves.",
            starting_abilities: vec![Ability::TimelinePerception],
            narrative_role: NarrativeRole::QuantumEntity,
            starting_goals: vec![Goal::new("Maintain Coherence".to_string(), 1.0, true)],
        },
        ProtagonistProfile {
            name: "Riven Blackwood",
            title: "The Gunslinger",
            description: "Bounty hunter with a semi-sentient Precursor revolver that fires \
                         bullets backward through time. Being hunted by their own future self.",
            starting_abilities: vec![Ability::CausalityHacking],
            narrative_role: NarrativeRole::CausalityManipulator,
            starting_goals: vec![Goal::new("Survive Future Self".to_string(), 1.0, true)],
        },
        ProtagonistProfile {
            name: "The Cartographer",
            title: "The Ring Historian",
            description: "Obsessively mapping Dead Zones and causality failures. Immune to \
                         memory manipulation; remembers the original timeline before the Incoherence.",
            starting_abilities: vec![Ability::MemoryImmunity, Ability::LoopMemory],
            narrative_role: NarrativeRole::HistoryKeeper,
            starting_goals: vec![Goal::new("Map All Dead Zones".to_string(), 0.8, false)],
        },
        ProtagonistProfile {
            name: "Synthesis",
            title: "The Hybrid Consciousness",
            description: "A merged entity of seven minds connected through illegal neural lace. \
                         Seven bodies, one mind (usually). The bodies are diverging into separate people.",
            starting_abilities: vec![],
            narrative_role: NarrativeRole::CollectiveEntity,
            starting_goals: vec![Goal::new("Achieve Individualism".to_string(), 0.5, false)],
        },
        ProtagonistProfile {
            name: "Mara Vex",
            title: "The Precognitive",
            description: "Afflicted with precognitive flashes showing contradictory futures. \
                         Cannot distinguish which future will occur. Living Gates feed her information \
                         from her future selves.",
            starting_abilities: vec![Ability::Precognition],
            narrative_role: NarrativeRole::PrecognitiveOracle,
            starting_goals: vec![Goal::new("Find True Future".to_string(), 0.9, false)],
        },
        ProtagonistProfile {
            name: "Kor-Valeth",
            title: "The Time-Exiled Warrior",
            description: "A warrior from 1,000 years in the Ring's past. Anchored to their original \
                         time; slowly being pulled back. Carries Precursor activation codes.",
            starting_abilities: vec![],
            narrative_role: NarrativeRole::TemporalExile,
            starting_goals: vec![Goal::new("Return to Past".to_string(), 1.0, false)],
        },
        ProtagonistProfile {
            name: "Dr. Theo Lux",
            title: "The Reality Hacker",
            description: "Rogue physicist who treats spacetime like code. Can create localized \
                         causality inversions. Secretly caused the Great Incoherence.",
            starting_abilities: vec![Ability::CausalityHacking],
            narrative_role: NarrativeRole::RealityHacker,
            starting_goals: vec![Goal::new("Rewrite Reality".to_string(), 0.7, false)],
        },
        ProtagonistProfile {
            name: "The Conductor",
            title: "The Mysterious Unifier",
            description: "Unknown identity; appears differently to each protagonist. Exists \
                         simultaneously in all timelines. The only truly coherent entity.",
            starting_abilities: vec![
                Ability::TimelinePerception,
                Ability::Precognition,
                Ability::MemoryImmunity,
                Ability::LoopMemory,
            ],
            narrative_role: NarrativeRole::UniversalNexus,
            starting_goals: vec![Goal::new("Prevent Ring Collapse".to_string(), 1.0, true)],
        },
    ]
}

/// Get a protagonist's starting relationships with others
pub fn initialize_relationships(multiverse: &mut Multiverse, char_ids: &[CharacterId]) {
    // Vera and Corvus start as allies (both trying to understand the Incoherence)
    if char_ids.len() >= 5 {
        add_relationship(multiverse, char_ids[0], char_ids[4], RelationshipState::Allied);
    }

    // Khelis and The Cartographer are rivals (memory vs history)
    if char_ids.len() >= 8 {
        add_relationship(
            multiverse,
            char_ids[1],
            char_ids[7],
            RelationshipState::Distrustful,
        );
    }

    // Dr. Saros and Dr. Lux are hostile (Saros suspects Lux caused the Incoherence)
    if char_ids.len() >= 12 {
        add_relationship(
            multiverse,
            char_ids[2],
            char_ids[11],
            RelationshipState::Hostile,
        );
    }

    // Nameless and The Conductor have a mysterious connection
    if char_ids.len() >= 13 {
        add_relationship(
            multiverse,
            char_ids[3],
            char_ids[12],
            RelationshipState::Neutral,
        );
    }

    // Riven is being hunted by their future self (self-hostility!)
    // This would require duplicating Riven, which we'll handle in story scenarios

    // Synthesis views The Cartographer as an ally (shared interest in preserving coherence)
    if char_ids.len() >= 9 {
        add_relationship(
            multiverse,
            char_ids[8],
            char_ids[7],
            RelationshipState::Friendly,
        );
    }

    // Mara and Corvus are friendly (both receive prophecies)
    if char_ids.len() >= 10 {
        add_relationship(
            multiverse,
            char_ids[9],
            char_ids[4],
            RelationshipState::Friendly,
        );
    }
}

fn add_relationship(
    multiverse: &mut Multiverse,
    char1: CharacterId,
    char2: CharacterId,
    state: RelationshipState,
) {
    if let Some(c1) = multiverse.characters.get_mut(&char1) {
        c1.relationships.insert(char2, state);
    }
    if let Some(c2) = multiverse.characters.get_mut(&char2) {
        c2.relationships.insert(char1, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thirteen_protagonists() {
        let mut multiverse = Multiverse::new();
        let char_ids = create_thirteen_protagonists(&mut multiverse);

        assert_eq!(char_ids.len(), 13);
        assert_eq!(multiverse.characters.len(), 13);

        // Verify Vera has TimelinePerception
        assert!(multiverse.characters[&char_ids[0]]
            .abilities
            .contains(&Ability::TimelinePerception));

        // Verify Nameless has both TimelinePerception and LoopMemory
        assert!(multiverse.characters[&char_ids[3]]
            .abilities
            .contains(&Ability::TimelinePerception));
        assert!(multiverse.characters[&char_ids[3]]
            .abilities
            .contains(&Ability::LoopMemory));
    }

    #[test]
    fn test_protagonist_names() {
        assert_eq!(PROTAGONIST_NAMES.len(), 13);
        assert_eq!(PROTAGONIST_NAMES[0], "Vera Kandros");
        assert_eq!(PROTAGONIST_NAMES[12], "The Conductor");
    }
}
