//! # Integration Tests: The Thirteen Suns Narrative in Action
//!
//! This module contains integration tests that demonstrate the property-based
//! testing framework applied to scenarios from "The Thirteen Suns" narrative.
//!
//! ## What We're Testing
//!
//! We generate random story sequences involving the 13 protagonists:
//! - Vera (Fold Captain with timeline superposition)
//! - Khelis (Memory Merchant)
//! - Dr. Saros (Probabilist)
//! - Nameless (Gate-Touched causality paradox)
//! - Corvus (Lattice Singer)
//! - Yash-Tel (Shimmer Navigator)
//! - Riven (Gunslinger with time-gun)
//! - The Cartographer
//! - Synthesis (7-body hybrid)
//! - Mara Vex (Precognitive)
//! - Kor-Valeth (Time-displaced warrior)
//! - Dr. Theo Lux (Reality hacker)
//! - The Conductor (multiversal entity)
//!
//! And verify that narrative invariants hold across ALL randomly-generated scenarios.

use crate::generators::*;
use crate::narrative_core::*;
use crate::properties::*;
use proptest::prelude::*;

#[cfg(test)]
proptest! {
    // ## Test: Memory Trading with the Memory Cartels
    //
    // Scenario: Khelis Tev trades memories in the Dark Spoke.
    // Property: All traded memories must have explicit provenance.
    #[test]
    fn test_memory_cartel_trading(
        num_trades in 1usize..20,
        memory_ids in prop::collection::vec(memory_id_strategy(), 1..20)
    ) {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;

        // Create Khelis and a customer
        let khelis = multiverse.create_character("Khelis Tev".to_string(), timeline);
        let customer = multiverse.create_character("Customer".to_string(), timeline);

        // Perform memory trades
        for i in 0..num_trades.min(memory_ids.len()) {
            apply_narrative_action(
                &mut multiverse,
                &NarrativeAction::TradeMemory {
                    memory: memory_ids[i],
                    from: khelis,
                    to: customer,
                    mechanism: "Memory Market".to_string(),
                },
            );
        }

        // Validate: all memory trades must be justified
        prop_assert!(validate_all_properties(&multiverse).is_ok());
    }
}

#[cfg(test)]
proptest! {
    // ## Test: Timeline Branching with the Fold Drive
    //
    // Scenario: Vera Kandros experiences timeline branches due to Fold Drive causality bubbles.
    // Property: Timeline branches must preserve history up to the divergence point.
    #[test]
    fn test_fold_drive_timeline_branching(num_branches in 1usize..5) {
        let mut multiverse = Multiverse::new();
        let root = multiverse.root_timeline;

        // Create Vera in root timeline
        let vera = multiverse.create_character("Vera Kandros".to_string(), root);

        // Grant Vera timeline perception ability
        if let Some(v) = multiverse.characters.get_mut(&vera) {
            v.abilities.insert(Ability::TimelinePerception);
        }

        // Vera makes decisions that branch timelines
        let mut current_timeline = root;
        for i in 0..num_branches {
            // Create a decision event
            let event_id = multiverse.record_event(Event {
                id: EventId(0),
                timeline: current_timeline,
                description: format!("Vera makes decision #{}", i),
                participants: vec![vera].into_iter().collect(),
                effects: vec![],
                causality_violation: None,
            });

            // Branch timeline
            let new_timeline = multiverse.create_timeline_branch(current_timeline, event_id);

            // Update Vera's timeline
            if let Some(v) = multiverse.characters.get_mut(&vera) {
                v.current_timeline = new_timeline;
            }

            current_timeline = new_timeline;
        }

        // Validate: Vera can perceive all timelines due to her ability
        prop_assert!(validate_all_properties(&multiverse).is_ok());
        prop_assert_eq!(multiverse.timelines.len(), 1 + num_branches);
    }
}

#[cfg(test)]
proptest! {
    // ## Test: Death and Resurrection via Living Gates
    //
    // Scenario: Nameless dies and is resurrected by a Living Gate.
    // Property: Dead characters can't act until resurrection event occurs.
    #[test]
    fn test_gate_resurrection(actions_before_death in 1usize..5, actions_after_resurrection in 1usize..5) {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;

        // Create Nameless
        let nameless = multiverse.create_character("Nameless".to_string(), timeline);

        // Nameless acts while alive
        for i in 0..actions_before_death {
            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Nameless action #{}", i),
                participants: vec![nameless].into_iter().collect(),
                effects: vec![],
                causality_violation: None,
            });
        }

        // Nameless dies
        multiverse.record_event(Event {
            id: EventId(0),
            timeline,
            description: "Nameless dies".to_string(),
            participants: vec![nameless].into_iter().collect(),
            effects: vec![EventEffect::CharacterDeath { character: nameless }],
            causality_violation: None,
        });

        // Living Gate resurrects Nameless
        multiverse.record_event(Event {
            id: EventId(0),
            timeline,
            description: "Living Gate resurrects Nameless".to_string(),
            participants: vec![nameless].into_iter().collect(),
            effects: vec![EventEffect::CharacterResurrection {
                character: nameless,
                mechanism: "Living Gate".to_string(),
            }],
            causality_violation: None,
        });

        // Nameless acts again after resurrection
        for i in 0..actions_after_resurrection {
            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Nameless post-resurrection action #{}", i),
                participants: vec![nameless].into_iter().collect(),
                effects: vec![],
                causality_violation: None,
            });
        }

        // Validate: death finality is respected
        prop_assert!(validate_all_properties(&multiverse).is_ok());
    }
}

#[cfg(test)]
proptest! {
    // ## Test: Causality Violation via Riven's Time-Gun
    //
    // Scenario: Riven Blackwood shoots their time-gun (effect precedes cause).
    // Property: Causality violations must have explicit mechanisms and mark timeline as unstable.
    #[test]
    fn test_time_gun_causality_violation(num_shots in 1usize..5) {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;

        // Create Riven
        let riven = multiverse.create_character("Riven Blackwood".to_string(), timeline);

        // Grant causality hacking ability
        if let Some(r) = multiverse.characters.get_mut(&riven) {
            r.abilities.insert(Ability::CausalityHacking);
        }

        // Riven fires time-gun multiple times
        for i in 0..num_shots {
            // Mark timeline as unstable
            if let Some(t) = multiverse.timelines.get_mut(&timeline) {
                t.causality_stable = false;
            }

            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Riven fires time-gun #{}", i),
                participants: vec![riven].into_iter().collect(),
                effects: vec![],
                causality_violation: Some(CausalityViolation::EffectBeforeCause {
                    mechanism: "Precursor Time-Weapon".to_string(),
                }),
            });
        }

        // Validate: causality violations are justified
        prop_assert!(validate_all_properties(&multiverse).is_ok());
    }
}

#[cfg(test)]
proptest! {
    // ## Test: Relationship Dynamics Between Protagonists
    //
    // Scenario: Characters form alliances and conflicts.
    // Property: Relationship changes must be reflected in subsequent interactions.
    #[test]
    fn test_protagonist_relationships(
        relationship_changes in prop::collection::vec(
            (relationship_state_strategy(), relationship_state_strategy()),
            1..10
        )
    ) {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;

        // Create two protagonists
        let vera = multiverse.create_character("Vera Kandros".to_string(), timeline);
        let corvus = multiverse.create_character("Corvus Shal".to_string(), timeline);

        // Apply relationship changes
        for (i, (state1, state2)) in relationship_changes.iter().enumerate() {
            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Relationship change #{}", i),
                participants: vec![vera, corvus].into_iter().collect(),
                effects: vec![
                    EventEffect::RelationshipChange {
                        character1: vera,
                        character2: corvus,
                        new_state: *state1,
                    },
                ],
                causality_violation: None,
            });

            // Later event with reversed relationship
            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Relationship change #{} reversed", i),
                participants: vec![vera, corvus].into_iter().collect(),
                effects: vec![
                    EventEffect::RelationshipChange {
                        character1: vera,
                        character2: corvus,
                        new_state: *state2,
                    },
                ],
                causality_violation: None,
            });
        }

        // Validate: relationship consistency is maintained
        prop_assert!(validate_all_properties(&multiverse).is_ok());
    }
}

#[cfg(test)]
proptest! {
    // ## Test: Knowledge Propagation Through the Ansible Lattice
    //
    // Scenario: Corvus Shal (Lattice Singer) shares knowledge through the ansible network.
    // Property: Knowledge flags must be set via explicit events.
    #[test]
    fn test_lattice_knowledge_sharing(
        knowledge_flags in prop::collection::vec("[a-z_]{5,15}", 1..10)
    ) {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;

        // Create Corvus and a recipient
        let corvus = multiverse.create_character("Corvus Shal".to_string(), timeline);
        let recipient = multiverse.create_character("Recipient".to_string(), timeline);

        // Corvus gains knowledge via Lattice
        for flag in &knowledge_flags {
            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Corvus learns {} via Lattice", flag),
                participants: vec![corvus].into_iter().collect(),
                effects: vec![EventEffect::KnowledgeGained {
                    character: corvus,
                    flag: flag.clone(),
                }],
                causality_violation: None,
            });

            // Corvus shares with recipient
            multiverse.record_event(Event {
                id: EventId(0),
                timeline,
                description: format!("Corvus shares {} with recipient", flag),
                participants: vec![corvus, recipient].into_iter().collect(),
                effects: vec![EventEffect::KnowledgeGained {
                    character: recipient,
                    flag: flag.clone(),
                }],
                causality_violation: None,
            });
        }

        // Validate: knowledge flags are properly tracked
        prop_assert!(validate_all_properties(&multiverse).is_ok());

        // Verify both characters have all knowledge
        let corvus_char = &multiverse.characters[&corvus];
        let recipient_char = &multiverse.characters[&recipient];

        for flag in &knowledge_flags {
            prop_assert!(corvus_char.knowledge_flags.contains(flag));
            prop_assert!(recipient_char.knowledge_flags.contains(flag));
        }
    }
}

#[cfg(test)]
proptest! {
    // ## Comprehensive Chaos Test: Random Action Sequences
    //
    // This test generates completely random sequences of narrative actions
    // and verifies that ALL properties hold throughout.
    //
    // This is the ultimate stress test—if properties hold here, they hold everywhere.
    #[test]
    fn test_random_narrative_sequences(
        actions in prop::collection::vec(narrative_action_strategy(), 10..50)
    ) {
        let mut multiverse = Multiverse::new();

        // Create the 13 protagonists
        let _vera = multiverse.create_character("Vera Kandros".to_string(), multiverse.root_timeline);
        let _khelis = multiverse.create_character("Khelis Tev".to_string(), multiverse.root_timeline);
        let _saros = multiverse.create_character("Dr. Elian Saros".to_string(), multiverse.root_timeline);
        let _nameless = multiverse.create_character("Nameless".to_string(), multiverse.root_timeline);
        let _corvus = multiverse.create_character("Corvus Shal".to_string(), multiverse.root_timeline);
        let _yash = multiverse.create_character("Yash-Tel".to_string(), multiverse.root_timeline);
        let _riven = multiverse.create_character("Riven Blackwood".to_string(), multiverse.root_timeline);
        let _cartographer = multiverse.create_character("The Cartographer".to_string(), multiverse.root_timeline);
        let _synthesis = multiverse.create_character("Synthesis".to_string(), multiverse.root_timeline);
        let _mara = multiverse.create_character("Mara Vex".to_string(), multiverse.root_timeline);
        let _kor = multiverse.create_character("Kor-Valeth".to_string(), multiverse.root_timeline);
        let _lux = multiverse.create_character("Dr. Theo Lux".to_string(), multiverse.root_timeline);
        let _conductor = multiverse.create_character("The Conductor".to_string(), multiverse.root_timeline);

        // Apply random actions
        for action in &actions {
            apply_narrative_action(&mut multiverse, action);

            // After EVERY action, properties must hold
            // This is the key insight: invariants are ALWAYS true, not just at endpoints
            if let Err(e) = validate_all_properties(&multiverse) {
                // If a property fails, proptest will shrink to minimal failing case
                panic!("Property violation after action {:?}: {}", action, e);
            }
        }

        // Final validation
        prop_assert!(validate_all_properties(&multiverse).is_ok());
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_thirteen_protagonists_creation() {
        let mut multiverse = Multiverse::new();
        let timeline = multiverse.root_timeline;

        // Create all 13 protagonists
        let protagonists = vec![
            "Vera Kandros",
            "Khelis Tev",
            "Dr. Elian Saros",
            "Nameless",
            "Corvus Shal",
            "Yash-Tel",
            "Riven Blackwood",
            "The Cartographer",
            "Synthesis",
            "Mara Vex",
            "Kor-Valeth",
            "Dr. Theo Lux",
            "The Conductor",
        ];

        for name in protagonists {
            multiverse.create_character(name.to_string(), timeline);
        }

        assert_eq!(multiverse.characters.len(), 13);
        assert!(validate_all_properties(&multiverse).is_ok());
    }
}
