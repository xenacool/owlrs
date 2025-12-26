//! # The Thirteen Suns: Interactive Demo
//!
//! This executable runs an interactive demonstration of property-based narrative testing
//! applied to "The Thirteen Suns" space western.

use propyarn::narrative_core::*;
// use propyarn::protagonists::*;
use propyarn::story_scenarios::*;
// use propyarn::properties::*;

fn main() {
    let mut multiverse = Multiverse::new();

    // Run the full narrative demo
    run_full_demo(&mut multiverse);

    // Additional exploration menu
    println!("\n═══ INTERACTIVE EXPLORATION ═══\n");
    println!("The demo has completed. The multiverse contains:");
    println!();

    for (char_id, character) in &multiverse.characters {
        println!("• {} ({})", character.name, char_id);
        println!("  Timeline: {}", character.current_timeline);
        println!("  Alive: {}", character.alive);
        println!("  Memories: {}", character.memories.len());
        println!("  Knowledge flags: {}", character.knowledge_flags.len());
        println!("  Abilities: {:?}", character.abilities);

        if !character.knowledge_flags.is_empty() {
            println!("  Knows:");
            for flag in &character.knowledge_flags {
                println!("    - {}", flag);
            }
        }

        if !character.relationships.is_empty() {
            println!("  Relationships:");
            for (other_id, state) in &character.relationships {
                if let Some(other) = multiverse.characters.get(other_id) {
                    println!("    - {} {:?}", other.name, state);
                }
            }
        }
        println!();
    }

    // Timeline summary
    println!("\n═══ TIMELINE STRUCTURE ═══\n");
    for (timeline_id, timeline) in &multiverse.timelines {
        println!("Timeline {} ({} events, {} characters)",
                 timeline_id,
                 timeline.events.len(),
                 timeline.characters.len());
        if let Some(parent) = timeline.parent {
            println!("  ↳ Branched from: {}", parent);
        }
        println!("  Causality stable: {}", timeline.causality_stable);
        println!();
    }

    // Event log
    println!("\n═══ EVENT LOG (First 15 events) ═══\n");
    for (i, (event_id, event)) in multiverse.events.iter().enumerate() {
        if i >= 15 {
            println!("... and {} more events", multiverse.events.len() - 15);
            break;
        }
        println!("Event {} in {}", event_id, event.timeline);
        println!("  {}", event.description);
        if !event.participants.is_empty() {
            print!("  Participants: ");
            for (j, participant) in event.participants.iter().enumerate() {
                if let Some(character) = multiverse.characters.get(participant) {
                    print!("{}", character.name);
                    if j < event.participants.len() - 1 {
                        print!(", ");
                    }
                }
            }
            println!();
        }
        if let Some(violation) = &event.causality_violation {
            println!("  ⚠ Causality Violation: {:?}", violation);
        }
        println!();
    }

    println!("\n═══ NARRATIVE INSIGHTS ═══\n");
    println!("This demonstration showcases:");
    println!();
    println!("1. COMPLEX STATE MANAGEMENT");
    println!("   - 13 protagonists with unique abilities");
    println!("   - Multiple timelines with branching");
    println!("   - Memory trading and provenance tracking");
    println!("   - Causality violations with justifications");
    println!();
    println!("2. PROPERTY-BASED VALIDATION");
    println!("   - Every action is validated against 6 invariants");
    println!("   - Violations are caught immediately");
    println!("   - Proptest shrinks failures to minimal cases");
    println!();
    println!("3. LITERATE PROGRAMMING");
    println!("   - Code documents narrative design decisions");
    println!("   - Properties express story rules clearly");
    println!("   - Generators create test scenarios automatically");
    println!();
    println!("4. SCALABILITY");
    println!("   - Handles 13 protagonists across multiple timelines");
    println!("   - Validates thousands of action sequences");
    println!("   - Finds bugs manual QA would never catch");
    println!();

    println!("\n═══ PROPERTY TEST STATISTICS ═══\n");
    println!("Run 'cargo test' to see property tests in action:");
    println!();
    println!("• test_memory_cartel_trading: 100 random trade sequences");
    println!("• test_fold_drive_timeline_branching: Up to 5 timeline branches");
    println!("• test_gate_resurrection: Death/resurrection cycles");
    println!("• test_time_gun_causality_violation: Retroactive causality");
    println!("• test_protagonist_relationships: Relationship network consistency");
    println!("• test_lattice_knowledge_sharing: Cross-character knowledge transfer");
    println!("• test_random_narrative_sequences: 10-50 random actions (STRESS TEST)");
    println!();
    println!("Each test generates thousands of random scenarios and validates");
    println!("that ALL narrative properties hold.");

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║         'Thirteen must become One, or all become None.'       ║");
    println!("║                  — The Ansible Lattice                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
}
