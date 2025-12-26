# PropYarn: Property-Based Testing for Narrative Systems

**A literate implementation demonstrating how to use property-based testing to validate complex interactive narratives**

## What This Is

This project implements a property-testing framework for "The Thirteen Suns"—a space western with 13 protagonists navigating broken causality, memory trading, timeline branching, and reality manipulation.

It proves that **property-based testing** (via Rust's `proptest`) can catch narrative inconsistencies that would take years of manual playtesting to find.

## The Problem

Modern interactive narratives are impossibly complex:
- 13 protagonists with interwoven storylines
- Multiple branching timelines
- Memory trading (extract, trade, forge, install)
- Causality violations (time travel, retroactive changes)
- Cross-timeline knowledge transfer
- Character deaths/resurrections across branches

Traditional testing checks specific scenarios. **Property testing checks invariants across thousands of randomly-generated scenarios.**

## The Solution

We define narrative invariants as executable properties:

1. **Memory Consistency**: Characters only remember events they witnessed OR acquired via trade
2. **Timeline Isolation**: Timeline-specific knowledge doesn't bleed across branches
3. **Causality Justification**: Causality violations require explicit mechanisms (Gates, time weapons)
4. **Relationship Persistence**: Relationships change only via explicit events
5. **Death Finality**: Dead characters can't act without resurrection
6. **Knowledge Propagation**: Knowledge flags are set via explicit events

Then we:
1. Generate thousands of random story sequences
2. Check all properties after every action
3. When a property fails, proptest **shrinks** to the minimal failing case

## Success: We Found a Bug!

Running `cargo test` immediately found a narrative bug:

```
Property violation after action KillCharacter { character: CharacterId(0), timeline: TimelineId(1) }:
Character Char#0 alive status is false but should be true based on timeline events.

minimal failing input: actions = [
    BranchTimeline { parent: TimelineId(0) },
    KillCharacter { character: CharacterId(0), timeline: TimelineId(1) },
]
```

**The Issue**: When a character dies in timeline 1, their `alive` status becomes globally false—even though they're still alive in timeline 0!

**The Fix**: Character alive status should be tracked per-timeline, not globally.

This is exactly the kind of bug that manual testing would miss—it only appears when timelines branch *and* a character dies in only one branch.

## Project Structure

```
src/
├── narrative_core.rs      # Core data structures (literate programming style)
├── properties.rs          # Property invariants that must always hold
├── generators.rs          # Proptest strategies for random scenario generation
├── integration_tests.rs   # Tests for "The Thirteen Suns" scenarios
└── lib.rs                 # Main documentation and module exports

notes/
├── space_western.md                          # Full world-building document
└── property-testing-narrative-conversation.md # The conversation that inspired this
```

## Running

```bash
# Run all tests (unit tests + property tests)
cargo test

# Run with more iterations (default is 100)
PROPTEST_CASES=1000 cargo test

# Run a specific test
cargo test test_memory_cartel_trading

# Build documentation
cargo doc --open
```

## Key Features

### Literate Programming

Code is heavily documented in narrative style, explaining *why* not just *what*:

```rust
/// ## Property 1: Memory Consistency
///
/// **Invariant**: A character can only have a memory of an event if:
/// - They witnessed the event (were present), OR
/// - They acquired the memory via explicit trade/installation
///
/// This prevents the common bug where characters mysteriously "know" things
/// they shouldn't.
#[cfg(test)]
pub fn prop_memory_consistency(multiverse: &Multiverse) -> Result<(), String> {
    // ... implementation
}
```

### Shrinking to Minimal Cases

When a property fails on a 50-action sequence, proptest automatically shrinks it to the minimal reproduction:

```
50-action sequence fails
→ Try 25 actions... still fails
→ Try 12 actions... still fails
→ Try 6 actions... still fails
→ Try 3 actions... still fails
→ Try 2 actions... SUCCESS! Minimal case found.
```

### The Thirteen Protagonists

The system models 13 protagonists from the space western:

1. **Vera Kandros** (Fold Captain): Experiences timeline branches simultaneously
2. **Khelis Tev** (Memory Merchant): Perfect memory of things that didn't happen to them
3. **Dr. Elian Saros** (Probabilist): Calculates future probability clouds
4. **Nameless** (Gate-Touched): A causality paradox who was never born
5. **Corvus Shal** (Lattice Singer): Hears the sapient ansible network
6. **Yash-Tel** (Shimmer Navigator): Exists in quantum superposition
7. **Riven Blackwood** (Gunslinger): Wields a gun that fires backward through time
8. **The Cartographer**: Immune to memory manipulation
9. **Synthesis**: Seven bodies, one distributed consciousness
10. **Mara Vex** (Precognitive): Sees contradictory futures
11. **Kor-Valeth** (Time-Exiled): Warrior from 1,000 years in the past
12. **Dr. Theo Lux** (Reality Hacker): Treats spacetime like code
13. **The Conductor**: Exists simultaneously in all timelines

## Example Test: Memory Cartel Trading

```rust
proptest! {
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
```

This test:
- Generates 1-20 random memory trades
- Applies them to the multiverse
- Verifies ALL properties still hold

## Inspiration

Based on a conversation between:
- **Joe Armstrong** (Erlang creator, property-testing advocate)
- **Leslie Lamport** (TLA+ creator, formal methods expert)
- **Robert Kurvitz** (Disco Elysium lead designer/writer)
- **Kate Compton** (Procedural generation researcher, Tracery creator)

Armstrong argued that property testing is more practical than formal verification (TLA+) for game narratives. This project proves the concept.

Full conversation: `notes/property-testing-narrative-conversation.md`

## The Narrative

"The Thirteen Suns" is a space western set on the Kaladrius Ring—a megastructure where reality itself has broken. The **Great Incoherence** has shattered causality:

- Effects sometimes precede causes
- Memories can be traded like currency
- Three incompatible FTL systems create paradoxes
- Timeline branches multiply uncontrollably

Thirteen protagonists must navigate this chaos, each perceiving reality differently.

Full world-building: `notes/space_western.md`

## Future Extensions

- **YarnSpinner Integration**: Generate property tests from narrative scripts
- **Literate Yarn**: Embed property specifications in dialogue files
- **Visual Shrinking**: Show minimal failing sequences graphically
- **Coverage Metrics**: Track which narrative paths have been tested
- **Mutation Testing**: Verify properties catch intentional bugs
- **Performance**: Cache property checks, incremental validation

## Why This Matters

Interactive narrative games are getting more complex:
- Disco Elysium: 1 million words, intricate skill system
- Baldur's Gate 3: Massive branching storylines
- Wildermyth: Procedural narratives with memory

Traditional QA can't catch all edge cases. Property-based testing provides:
- **Exhaustive exploration** of narrative state space
- **Automatic bug minimization** via shrinking
- **Living documentation** of narrative invariants
- **Confidence** to write ambitious, complex stories

## License

This is a demonstration/educational project exploring property-based testing for interactive narratives.

---

**"Thirteen must become One, or all become None."**
— The Ansible Lattice prophecy
