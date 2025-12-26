//! # Story Scenarios: Implementing Story Threads from "The Thirteen Suns"
//!
//! This module implements the four major story threads described in space_western.md:
//!
//! - **Thread Alpha**: The Memory of God (Khelis acquires Precursor memory)
//! - **Thread Beta**: The Gunslinger's Paradox (Riven hunted by future self)
//! - **Thread Gamma**: The Shimmer Convergence (Yash-Tel merging with parallel selves)
//! - **Thread Delta**: The Lattice Prophecy (Thirteen must become One)
//!
//! Each scenario demonstrates complex narrative properties being validated.

use crate::narrative_core::*;
use crate::protagonists::*;
use crate::properties::*;
use crate::emotional_system::*;

/// ## Thread Alpha: The Memory of God
///
/// **Story**: Khelis Tev acquires a Precursor memory showing the Ring's original purpose.
/// Vera Kandros must transport Khelis to Foundation Town so Dr. Saros can decode it.
/// But The Cartographer warns the memory is a forgery planted by the Gate Cult.
///
/// **Branching Point**: Trust Cartographer (destroy memory), trust Conductor (trade memory),
/// or trust Saros (decode it).
pub fn thread_alpha_memory_of_god(multiverse: &mut Multiverse, char_ids: &[CharacterId]) {
    let khelis = char_ids[1]; // Memory Merchant
    let vera = char_ids[0]; // Fold Captain
    let _saros = char_ids[2]; // Probabilist
    let cartographer = char_ids[7]; // Ring Historian
    let conductor = char_ids[12]; // Mysterious Unifier

    let timeline = multiverse.root_timeline;

    // === ACT 1: Khelis acquires the Precursor memory ===

    // Event: Khelis discovers a Precursor memory crystal in the Dark Spoke
    let discovery_event = multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Khelis discovers a Precursor memory crystal glowing with impossible colors"
            .to_string(),
        participants: vec![khelis].into_iter().collect(),
        effects: vec![EventEffect::AppraisalTrigger {
            character: khelis,
            belief: Belief {
                likelihood: 1.0,
                causal_agent_name: None,
                affected_goal_names: vec!["Acquire Unique Memories".to_string()],
                goal_congruences: vec![0.5], // Progress towards the goal
                is_incremental: true,
            },
        }],
        causality_violation: None,
    });

    // Create the Precursor memory (source is from before the Incoherence)
    let precursor_memory = Memory {
        id: MemoryId(1000),
        event: discovery_event,
        source_timeline: timeline,
        provenance: MemoryProvenance::Forged {
            forger: "Unknown Precursor Entity".to_string(),
        },
        fidelity: 1.0, // Perfect fidelity but potentially false!
    };
    multiverse.memories.insert(precursor_memory.id, precursor_memory);

    // Khelis installs the memory
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Khelis installs the Precursor memory. Visions flood their mind: \
                     the Ring is a causality engine designed to stabilize reality itself."
            .to_string(),
        participants: vec![khelis].into_iter().collect(),
        effects: vec![
            EventEffect::MemoryTransfer {
                memory: MemoryId(1000),
                from: None,
                to: khelis,
            },
            EventEffect::KnowledgeGained {
                character: khelis,
                flag: "knows_ring_purpose".to_string(),
            },
        ],
        causality_violation: None,
    });

    // === ACT 2: Vera agrees to transport Khelis to Foundation Town ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Khelis contacts Vera Kandros. 'I have something that could save us all. \
                     I need passage to Foundation Town.'"
            .to_string(),
        participants: vec![khelis, vera].into_iter().collect(),
        effects: vec![
            EventEffect::RelationshipChange {
                character1: khelis,
                character2: vera,
                new_state: RelationshipState::Friendly,
            },
            EventEffect::AppraisalTrigger {
                character: vera,
                belief: Belief {
                    likelihood: 0.2,
                    causal_agent_name: Some("Khelis Tev".to_string()),
                    affected_goal_names: vec!["Protect Crew".to_string()],
                    goal_congruences: vec![-0.1], // Taking a risk
                    is_incremental: true,
                },
            },
        ],
        causality_violation: None,
    });

    // === ACT 3: The Cartographer's Warning ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Cartographer intercepts them at Spinward Edge. 'That memory is a lie. \
                     I've mapped the original timeline. The Gates planted it to lure you.'"
            .to_string(),
        participants: vec![cartographer, khelis, vera].into_iter().collect(),
        effects: vec![
            EventEffect::KnowledgeGained {
                character: vera,
                flag: "cartographer_warns_forgery".to_string(),
            },
            EventEffect::KnowledgeGained {
                character: khelis,
                flag: "cartographer_warns_forgery".to_string(),
            },
        ],
        causality_violation: None,
    });

    // === ACT 4: The Conductor's Offer ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Conductor materializes aboard the Errant Promise. To Khelis: \
                     'Surrender the memory to me, and I will tell you who you were before \
                     you sold your identity.'"
            .to_string(),
        participants: vec![conductor, khelis, vera].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: khelis,
            flag: "conductor_offers_identity".to_string(),
        }],
        causality_violation: None,
    });

    // === BRANCHING POINT: Three possible choices ===

    // We'll create three timeline branches to represent the three choices
    // In a real game, player chooses which branch to follow

    println!("\n=== THREAD ALPHA: The Memory of God ===");
    println!("Khelis has acquired a Precursor memory.");
    println!("The Cartographer says it's a forgery.");
    println!("The Conductor offers Khelis their lost identity in exchange.");
    println!("\nThree timelines diverge from this moment...\n");
}

/// ## Thread Beta: The Gunslinger's Paradox
///
/// **Story**: Riven Blackwood is being hunted by their future self, who claims Riven
/// will accidentally kill Corvus Shal in three days, causing the Lattice to destroy
/// the Dark Spoke. But Mara Vex has seen a future where Corvus *must* die to prevent
/// worse catastrophe.
///
/// **Branching Point**: Trust future-Riven (avoid Corvus), trust Mara (kill Corvus),
/// or seek Kor-Valeth (learn the gun's true purpose).
pub fn thread_beta_gunslinger_paradox(multiverse: &mut Multiverse, char_ids: &[CharacterId]) {
    let riven = char_ids[6]; // Gunslinger
    let mara = char_ids[9]; // Precognitive
    let _kor_valeth = char_ids[10]; // Time-Exiled Warrior
    let corvus = char_ids[4]; // Lattice Singer

    let timeline = multiverse.root_timeline;

    // === ACT 1: Future-Riven appears ===

    // First, we need to create "Future-Riven" as a separate entity
    let future_riven = multiverse.create_character("Riven Blackwood (Future)".to_string(), timeline);

    // Grant Future-Riven the same abilities
    if let Some(fr) = multiverse.characters.get_mut(&future_riven) {
        fr.abilities.insert(Ability::CausalityHacking);
        fr.abilities.insert(Ability::TimelinePerception); // Knows the future
    }

    // Event: Future-Riven ambushes Present-Riven
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "A figure emerges from a temporal shimmer—Riven Blackwood, older, scarred. \
                     'In three days, you kill Corvus Shal. The Lattice retaliates. The Dark Spoke \
                     burns. Ten thousand die. I'm here to stop you.'"
            .to_string(),
        participants: vec![riven, future_riven].into_iter().collect(),
        effects: vec![
            EventEffect::KnowledgeGained {
                character: riven,
                flag: "warned_will_kill_corvus".to_string(),
            },
            EventEffect::RelationshipChange {
                character1: riven,
                character2: corvus,
                new_state: RelationshipState::Neutral, // Riven now wary
            },
        ],
        causality_violation: Some(CausalityViolation::EffectBeforeCause {
            mechanism: "Precursor Time-Weapon (Future-Riven's gun)".to_string(),
        }),
    });

    // Mark timeline as causality-unstable due to time travel
    if let Some(t) = multiverse.timelines.get_mut(&timeline) {
        t.causality_stable = false;
    }

    // === ACT 2: Mara's Contradictory Vision ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Mara Vex finds Riven in the Singing Gardens. Her eyes are distant, \
                     seeing futures. 'I've seen what happens if Corvus lives. The Lattice \
                     achieves full consciousness. Humanity becomes... subsumed. Corvus must die, \
                     Riven. You must pull the trigger.'"
            .to_string(),
        participants: vec![mara, riven].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: riven,
            flag: "mara_says_must_kill_corvus".to_string(),
        }],
        causality_violation: None,
    });

    // === ACT 3: The Gun's Instructions ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Conductor appears with a crystalline data-tablet. 'The manual for \
                     your weapon. It explains what the gun truly does. But only Kor-Valeth can \
                     read Precursor script.' The Conductor vanishes."
            .to_string(),
        participants: vec![riven].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: riven,
            flag: "has_gun_manual".to_string(),
        }],
        causality_violation: None,
    });

    println!("\n=== THREAD BETA: The Gunslinger's Paradox ===");
    println!("Riven is hunted by their future self.");
    println!("Future-Riven says: Don't kill Corvus or thousands die.");
    println!("Mara Vex says: Kill Corvus or humanity is subsumed.");
    println!("The gun's manual might explain everything—if Kor-Valeth can translate it.\n");
}

/// ## Thread Gamma: The Shimmer Convergence
///
/// **Story**: Yash-Tel is merging with their parallel selves and losing coherent identity.
/// Synthesis offers neural lace stabilization (but it's failing too). Dr. Lux proposes
/// a causality hack (but it destabilizes the Ring). Nameless knows a Living Gate that
/// could "fix" Yash-Tel by rewriting their past (but erases all memories).
pub fn thread_gamma_shimmer_convergence(multiverse: &mut Multiverse, char_ids: &[CharacterId]) {
    let yash_tel = char_ids[5]; // Shimmer Navigator
    let synthesis = char_ids[8]; // Hybrid Consciousness
    let lux = char_ids[11]; // Reality Hacker
    let nameless = char_ids[3]; // Gate-Touched

    let timeline = multiverse.root_timeline;

    // === ACT 1: Yash-Tel's Condition Worsens ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Yash-Tel collapses in the Shimmer Bay. They're speaking in overlapping \
                     voices—their parallel selves bleeding through. 'I am/we are/they were here/ \
                     not here/never here.' Identity fracturing."
            .to_string(),
        participants: vec![yash_tel].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: yash_tel,
            flag: "identity_fragmenting".to_string(),
        }],
        causality_violation: Some(CausalityViolation::Superposition {
            mechanism: "Shimmer Path quantum entanglement".to_string(),
        }),
    });

    if let Some(t) = multiverse.timelines.get_mut(&timeline) {
        t.causality_stable = false;
    }

    // === ACT 2: Synthesis's Offer ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Synthesis extends seven hands toward Yash-Tel. 'We understand multiplicity. \
                     Our neural lace can anchor you—bind your selves into one coherent thread. \
                     But... our own unity is failing. This might kill us both.'"
            .to_string(),
        participants: vec![synthesis, yash_tel].into_iter().collect(),
        effects: vec![
            EventEffect::KnowledgeGained {
                character: yash_tel,
                flag: "synthesis_offers_lace".to_string(),
            },
            EventEffect::RelationshipChange {
                character1: yash_tel,
                character2: synthesis,
                new_state: RelationshipState::Friendly,
            },
        ],
        causality_violation: None,
    });

    // === ACT 3: Dr. Lux's Causality Hack ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Dr. Lux arrives with forbidden equations scrawled on transparent datasheets. \
                     'I can anchor you to a single timeline permanently. Rewrite spacetime so you \
                     never entered superposition. But it requires siphoning energy from the Ring's \
                     core. The Incoherence will accelerate. Days? Weeks? Hard to say.'"
            .to_string(),
        participants: vec![lux, yash_tel].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: yash_tel,
            flag: "lux_offers_causality_hack".to_string(),
        }],
        causality_violation: None,
    });

    // === ACT 4: Nameless and the Living Gate ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Nameless appears like a ghost. 'There's a Gate. Dormant. In the Veins. \
                     It could rewrite your history so you were never a Shimmer Navigator. Never \
                     entered superposition. But the Gate takes payment—it will erase your memories. \
                     All of them. You'll be... someone else.'"
            .to_string(),
        participants: vec![nameless, yash_tel].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: yash_tel,
            flag: "nameless_offers_gate".to_string(),
        }],
        causality_violation: None,
    });

    println!("\n=== THREAD GAMMA: The Shimmer Convergence ===");
    println!("Yash-Tel is fragmenting across parallel selves.");
    println!("Option 1: Synthesis's neural lace (risk: both die)");
    println!("Option 2: Dr. Lux's causality hack (risk: Ring destabilizes)");
    println!("Option 3: Living Gate rewrite (risk: lose all memories)\n");
}

/// ## Thread Delta: The Lattice Prophecy
///
/// **Story**: The Ansible Lattice tells Corvus Shal: "Thirteen must become One, or all
/// become None." The Conductor is gathering all thirteen protagonists. Multiple factions
/// interpret the prophecy differently.
pub fn thread_delta_lattice_prophecy(multiverse: &mut Multiverse, char_ids: &[CharacterId]) {
    let corvus = char_ids[4]; // Lattice Singer
    let _conductor = char_ids[12]; // Mysterious Unifier
    let saros = char_ids[2]; // Probabilist
    let cartographer = char_ids[7]; // Ring Historian

    let timeline = multiverse.root_timeline;

    // === ACT 1: The Lattice Speaks ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Corvus Shal sits in the Singing Gardens, interfacing with the Ansible Lattice. \
                     The network's voice fills their mind—layered, infinite, impossible. \
                     'THIRTEEN MUST BECOME ONE, OR ALL BECOME NONE.' The message repeats across \
                     every ansible in the Ring."
            .to_string(),
        participants: vec![corvus].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: corvus,
            flag: "heard_lattice_prophecy".to_string(),
        }],
        causality_violation: None,
    });

    // === ACT 2: The Conductor Begins Gathering ===

    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Conductor appears to each of the thirteen protagonists simultaneously \
                     (somehow). To each, they say: 'The Lattice has spoken. You must come to the \
                     center of the Ring. All thirteen. When the time is right.' Then they vanish."
            .to_string(),
        participants: char_ids.iter().copied().collect(),
        effects: char_ids
            .iter()
            .map(|&char_id| EventEffect::KnowledgeGained {
                character: char_id,
                flag: "conductor_summons".to_string(),
            })
            .collect(),
        causality_violation: Some(CausalityViolation::Superposition {
            mechanism: "The Conductor exists in all timelines simultaneously".to_string(),
        }),
    });

    if let Some(t) = multiverse.timelines.get_mut(&timeline) {
        t.causality_stable = false;
    }

    // === ACT 3: Competing Interpretations ===

    // Dr. Saros's interpretation
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "Dr. Saros runs probability calculations. 'The prophecy predicts timeline \
                     collapse. All branches converging to a single outcome. Thirteen causal \
                     threads becoming one coherent narrative. Or total dissolution.'"
            .to_string(),
        participants: vec![saros].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: saros,
            flag: "interprets_prophecy_mathematical".to_string(),
        }],
        causality_violation: None,
    });

    // The Cartographer's interpretation
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Cartographer consults their maps. 'I've seen this pattern before— \
                     in the original timeline. The Precursors left instructions. Thirteen \
                     individuals with specific ontological signatures must merge consciousness \
                     to reboot the Ring's causality engine.'"
            .to_string(),
        participants: vec![cartographer].into_iter().collect(),
        effects: vec![EventEffect::KnowledgeGained {
            character: cartographer,
            flag: "interprets_prophecy_historical".to_string(),
        }],
        causality_violation: None,
    });

    // The Foundation Collective's interpretation
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Foundation Collective broadcasts: 'The prophecy is a self-fulfilling \
                     prediction. Gathering the thirteen will CREATE the collapse. We must prevent \
                     the meeting.'"
            .to_string(),
        participants: vec![].into_iter().collect(),
        effects: vec![],
        causality_violation: None,
    });

    // The Gate Cult's interpretation
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Gate Cult prepares rituals: 'Thirteen souls must merge with the Living \
                     Gates. Become one with the network. Transcend linear existence.'"
            .to_string(),
        participants: vec![].into_iter().collect(),
        effects: vec![],
        causality_violation: None,
    });

    // The Causality Purists' interpretation
    multiverse.record_event(Event {
        id: EventId(0),
        timeline,
        description: "The Causality Purists issue a kill order: 'The thirteen are causality \
                     anomalies. They ARE the Incoherence. Eliminate them and reality stabilizes. \
                     Thirteen must become NONE.'"
            .to_string(),
        participants: vec![].into_iter().collect(),
        effects: vec![],
        causality_violation: None,
    });

    println!("\n=== THREAD DELTA: The Lattice Prophecy ===");
    println!("'THIRTEEN MUST BECOME ONE, OR ALL BECOME NONE.'");
    println!("The Conductor summons all thirteen protagonists.");
    println!("\nCompeting interpretations:");
    println!("- Foundation Collective: Don't gather (prevents collapse)");
    println!("- Gate Cult: Merge with Gates (transcendence)");
    println!("- Causality Purists: Kill the thirteen (elimination)");
    println!("- The Cartographer: Follow Precursor instructions (reboot)\n");
}

/// Run all four story threads in sequence, demonstrating narrative coherence
pub fn run_full_demo(multiverse: &mut Multiverse) {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║   THE THIRTEEN SUNS: A Property-Tested Interactive Narrative  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("Initializing the Kaladrius Ring...");
    println!("Creating thirteen protagonists...\n");

    let char_ids = create_thirteen_protagonists(multiverse);
    initialize_relationships(multiverse, &char_ids);

    // Display protagonist roster
    println!("═══ THE THIRTEEN PROTAGONISTS ═══\n");
    for (i, profile) in protagonist_profiles().iter().enumerate() {
        println!(
            "{}. {} - {}",
            i + 1,
            profile.name,
            profile.title
        );
        println!("   {}", profile.description);
        if !profile.starting_abilities.is_empty() {
            println!("   Abilities: {:?}", profile.starting_abilities);
        }
        println!();
    }

    // Run each story thread
    thread_alpha_memory_of_god(multiverse, &char_ids);
    thread_beta_gunslinger_paradox(multiverse, &char_ids);
    thread_gamma_shimmer_convergence(multiverse, &char_ids);
    thread_delta_lattice_prophecy(multiverse, &char_ids);

    // Validate all properties still hold
    println!("\n═══ NARRATIVE VALIDATION ═══\n");
    println!("Running property tests on full narrative state...");

    match validate_all_properties(multiverse) {
        Ok(()) => {
            println!("✓ All narrative properties hold!");
            println!("  - Memory consistency: PASS");
            println!("  - Timeline isolation: PASS");
            println!("  - Causality justification: PASS");
            println!("  - Relationship persistence: PASS");
            println!("  - Death finality: PASS");
            println!("  - Knowledge propagation: PASS");
        }
        Err(e) => {
            println!("✗ Property violation detected:");
            println!("  {}", e);
        }
    }

    // Display final state summary
    println!("\n═══ NARRATIVE STATE SUMMARY ═══\n");
    println!("Timelines: {}", multiverse.timelines.len());
    println!("Characters: {}", multiverse.characters.len());
    println!("Events recorded: {}", multiverse.events.len());
    println!("Memories in circulation: {}", multiverse.memories.len());

    let causality_unstable_count = multiverse
        .timelines
        .values()
        .filter(|t| !t.causality_stable)
        .count();
    println!("Causality-unstable timelines: {}", causality_unstable_count);

    println!("\n═══ PLAYER CHOICE POINTS ═══\n");
    println!("The narrative has reached four major branching points.");
    println!("Each represents a player decision that creates diverging timelines:");
    println!();
    println!("1. Memory of God: Destroy, trade, or decode the Precursor memory?");
    println!("2. Gunslinger's Paradox: Trust future-self, trust Mara, or consult Kor-Valeth?");
    println!("3. Shimmer Convergence: Accept lace, accept hack, or accept Gate rewrite?");
    println!("4. Lattice Prophecy: Gather the thirteen, or resist?");
    println!();
    println!("Property-based testing ensures that ALL possible combinations");
    println!("maintain narrative coherence across the state space.");

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                    END OF DEMONSTRATION                        ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_alpha_maintains_properties() {
        let mut multiverse = Multiverse::new();
        let char_ids = create_thirteen_protagonists(&mut multiverse);
        thread_alpha_memory_of_god(&mut multiverse, &char_ids);

        // All properties should still hold
        assert!(validate_all_properties(&multiverse).is_ok());
    }

    #[test]
    fn test_thread_beta_causality_justified() {
        let mut multiverse = Multiverse::new();
        let char_ids = create_thirteen_protagonists(&mut multiverse);
        thread_beta_gunslinger_paradox(&mut multiverse, &char_ids);

        // Causality violations should be justified
        assert!(prop_causality_justification(&multiverse).is_ok());
    }

    #[test]
    fn test_all_threads_coherent() {
        let mut multiverse = Multiverse::new();
        run_full_demo(&mut multiverse);

        // Even after all four story threads, narrative should be coherent
        assert!(validate_all_properties(&multiverse).is_ok());
    }
}
