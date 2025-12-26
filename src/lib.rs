//! # PropYarn: Property-Based Testing for Narrative Systems
//!
//! ## A Literate Implementation of "The Thirteen Suns"
//!
//! This crate demonstrates how to use property-based testing (via Rust's `proptest`)
//! to validate complex narrative systems with:
//!
//! - **13 protagonists** with interwoven storylines
//! - **Multiple timelines** that branch and diverge
//! - **Memory trading** where memories can be extracted, traded, and forged
//! - **Causality violations** (time travel, retroactive changes, etc.)
//! - **Relationship networks** across timeline branches
//! - **Reality manipulation** and ontological flexibility
//!
//! ## Why Property-Based Testing for Narratives?
//!
//! Traditional branching narratives explode in complexity. With 13 protagonists,
//! multiple timelines, and systems like memory trading, the state space becomes
//! astronomically large. Manual testing catches obvious bugs but misses edge cases.
//!
//! **Property-based testing solves this** by:
//!
//! 1. **Defining invariants** that must always hold (e.g., "dead characters can't speak")
//! 2. **Generating thousands** of random story sequences
//! 3. **Checking invariants** after every action
//! 4. **Shrinking failures** to minimal reproduction cases
//!
//! This catches bugs that would take years of manual playtesting to find.
//!
//! ## The Conversation That Inspired This
//!
//! Based on a dialogue between:
//! - **Joe Armstrong** (Erlang creator, property-testing advocate)
//! - **Leslie Lamport** (TLA+ creator, formal methods expert)
//! - **Robert Kurvitz** (Disco Elysium lead writer)
//! - **Kate Compton** (Procedural generation researcher, Tracery creator)
//!
//! Armstrong argued that property testing is more practical than formal verification
//! for game narratives. This crate proves the concept.
//!
//! ## The Narrative: The Thirteen Suns
//!
//! A space western where reality itself is broken. The Kaladrius Ring—a megastructure
//! orbiting a neutron star—is experiencing **The Great Incoherence**: causality no
//! longer flows linearly.
//!
//! Thirteen protagonists navigate this chaos:
//!
//! 1. **Vera Kandros** (Fold Captain): Experiences timeline branches simultaneously
//! 2. **Khelis Tev** (Memory Merchant): Perfect memory of things that didn't happen to them
//! 3. **Dr. Elian Saros** (Probabilist): Calculates future probability clouds
//! 4. **Nameless** (Gate-Touched): A causality paradox who was never born
//! 5. **Corvus Shal** (Lattice Singer): Hears the sapient ansible network
//! 6. **Yash-Tel** (Shimmer Navigator): Exists in quantum superposition
//! 7. **Riven Blackwood** (Gunslinger): Wields a gun that fires backward through time
//! 8. **The Cartographer**: Immune to memory manipulation
//! 9. **Synthesis**: Seven bodies, one distributed consciousness
//! 10. **Mara Vex** (Precognitive): Sees contradictory futures
//! 11. **Kor-Valeth** (Time-Exiled): Warrior from 1,000 years in the past
//! 12. **Dr. Theo Lux** (Reality Hacker): Treats spacetime like code
//! 13. **The Conductor**: Exists simultaneously in all timelines
//!
//! ## Module Structure
//!
//! - **`narrative_core`**: Core data structures (Timeline, Character, Memory, Event)
//! - **`properties`**: Property tests that validate narrative invariants
//! - **`generators`**: Proptest strategies for generating random scenarios
//! - **`integration_tests`**: Tests applying properties to "The Thirteen Suns"
//!
//! ## Example: Validating Memory Consistency
//!
//! ```rust,no_run
//! use propyarn::narrative_core::*;
//! use propyarn::properties::*;
//!
//! let mut multiverse = Multiverse::new();
//! let timeline = multiverse.root_timeline;
//!
//! // Create character
//! let character = multiverse.create_character("Alice".to_string(), timeline);
//!
//! // Character witnesses an event
//! let event_id = multiverse.record_event(Event {
//!     id: EventId(0),
//!     timeline,
//!     description: "Alice sees something".to_string(),
//!     participants: vec![character].into_iter().collect(),
//!     effects: vec![],
//!     causality_violation: None,
//! });
//!
//! // Create memory
//! let memory_id = multiverse.create_witnessed_memory(event_id, timeline, character);
//!
//! // Add memory to character
//! if let Some(c) = multiverse.characters.get_mut(&character) {
//!     c.memories.insert(memory_id);
//! }
//!
//! // Validate: memory consistency should hold
//! assert!(prop_memory_consistency(&multiverse).is_ok());
//! ```
//!
//! ## Running Tests
//!
//! ```bash
//! cargo test
//! ```
//!
//! This will run:
//! - Unit tests validating individual components
//! - Property tests generating thousands of random scenarios
//! - Integration tests applying properties to "The Thirteen Suns"
//!
//! ## Narrative Design Philosophy
//!
//! This system embodies several design principles:
//!
//! ### 1. Explicit Justification
//!
//! Everything that happens must have an in-universe justification:
//! - Memory of an event? Either witnessed or traded
//! - Causality violation? Must have a mechanism (Gates, time weapon)
//! - Dead character acting? Must have resurrection event
//!
//! ### 2. Timeline Isolation
//!
//! Knowledge from one timeline doesn't bleed into others unless:
//! - Character has timeline-perception ability
//! - Knowledge was transferred via explicit mechanism (Lattice network, etc.)
//!
//! ### 3. Relationship Persistence
//!
//! Within a timeline, relationships change only via explicit events.
//! You can't be friendly in one scene and hostile in the next without justification.
//!
//! ### 4. State Visibility
//!
//! All narrative state is explicit and trackable:
//! - What does this character know?
//! - What timeline are they in?
//! - Are they alive?
//! - What memories do they have?
//!
//! ### 5. Composition Over Constraint
//!
//! Rather than limiting what authors can write, we validate what they *have* written.
//! Want time travel? Causality violations? Memory forgery? Go ahead—just make sure
//! it's justified and consistent.
//!
//! ## Inspirations
//!
//! This system draws from:
//!
//! - **[space romance pulp]**: Planetary romance, transformation, alien psychology
//! - **[reality-bender guy]**: Reality mutability, paranoia, precognitive fragments
//! - **[robot trilogy person]**: Statistical prediction of galactic futures
//! - **[big ring enthusiast]**: Megastructures, deep time, ancient mysteries
//! - **[ansible inventor]**: Instantaneous communication, anthropological depth
//!
//! ## Future Work
//!
//! Potential extensions:
//!
//! - **YarnSpinner integration**: Generate property tests from narrative scripts
//! - **Literate Yarn**: Embed property specifications in dialogue files
//! - **Shrinking visualizer**: Show minimal failing sequences graphically
//! - **Coverage metrics**: Track which narrative paths have been tested
//! - **Mutation testing**: Verify properties catch intentional bugs
//! - **Performance optimization**: Cache property checks, incremental validation
//!
//! ## License
//!
//! This is a demonstration/educational project exploring property-based testing
//! for interactive narratives.

pub mod narrative_core;
pub mod emotional_system;
pub mod properties;
pub mod generators;
#[cfg(test)]
pub mod integration_tests;
pub mod protagonists;
pub mod story_scenarios;

pub use narrative_core::*;
pub use emotional_system::*;
pub use properties::*;
pub use generators::*;
pub use protagonists::*;
pub use story_scenarios::*;
