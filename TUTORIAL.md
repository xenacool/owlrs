# Tutorial: Property-Based Testing for Interactive Narratives

## Introduction

This tutorial walks through how to use property-based testing to validate a complex interactive narrative, using "The Thirteen Suns" as an example.

## Part 1: Understanding the Problem

### Traditional Narrative Testing

```
Test: "If player kills Kim, dialogue should acknowledge his death"
→ Run playthrough where Kim dies
→ Check that death-acknowledgment dialogue appears
✓ Test passes
```

**Problem**: You only tested ONE scenario. What about:
- Kim dies in timeline A but is alive in timeline B?
- Kim dies, then is resurrected by a Living Gate?
- Kim dies in the past via retroactive causality change?
- Kim's death is in another character's traded memory?

### Property-Based Testing

```
Property: "Dead characters cannot speak (unless resurrected)"
→ Generate 10,000 random story sequences
→ Check property holds after EVERY action
→ When it fails, shrink to minimal reproduction
✓ Catches edge cases you never imagined
```

## Part 2: Defining Core Data Structures

### Step 1: Model Your Narrative State

```rust
pub struct Multiverse {
    pub timelines: HashMap<TimelineId, Timeline>,
    pub characters: HashMap<CharacterId, Character>,
    pub memories: HashMap<MemoryId, Memory>,
    pub events: HashMap<EventId, Event>,
}
```

**Key insight**: Everything is explicit and trackable. No hidden state.

### Step 2: Track Character State

```rust
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub current_timeline: TimelineId,
    pub memories: HashSet<MemoryId>,
    pub knowledge_flags: HashSet<String>,
    pub alive: bool,
    pub abilities: HashSet<Ability>,
    pub relationships: HashMap<CharacterId, RelationshipState>,
}
```

**Key insight**: Every piece of state that affects dialogue/narrative must be here.

### Step 3: Model Events

```rust
pub struct Event {
    pub id: EventId,
    pub timeline: TimelineId,
    pub description: String,
    pub participants: HashSet<CharacterId>,
    pub effects: Vec<EventEffect>,
    pub causality_violation: Option<CausalityViolation>,
}
```

**Key insight**: Events are first-class entities with explicit effects.

## Part 3: Defining Properties

### Property 1: Memory Consistency

**Invariant**: "Characters only remember events they witnessed OR acquired via trade"

```rust
pub fn prop_memory_consistency(multiverse: &Multiverse) -> Result<(), String> {
    for (char_id, character) in &multiverse.characters {
        for memory_id in &character.memories {
            let memory = multiverse.memories.get(memory_id)?;

            match &memory.provenance {
                MemoryProvenance::Witnessed { character: witness } => {
                    // Verify character was actually present
                    let event = multiverse.events.get(&memory.event)?;
                    if !event.participants.contains(witness) {
                        return Err(format!("{} has memory but wasn't present", char_id));
                    }
                }
                MemoryProvenance::Traded { .. } => {
                    // Trade is justification enough
                }
                // ... other cases
            }
        }
    }
    Ok(())
}
```

**Why this matters**: Prevents "How do you know that?" bugs where characters mysteriously have knowledge they shouldn't.

### Property 2: Death Finality

**Invariant**: "Dead characters can't participate in events (unless resurrected)"

```rust
pub fn prop_death_finality(multiverse: &Multiverse) -> Result<(), String> {
    let mut alive_state = HashMap::new();

    for timeline in multiverse.timelines.values() {
        for event_id in &timeline.events {
            let event = multiverse.events.get(event_id)?;

            // Check all participants are alive
            for participant in &event.participants {
                if !alive_state.get(participant).unwrap_or(true) {
                    // Unless this is a resurrection event
                    let is_resurrection = event.effects.iter().any(|e| {
                        matches!(e, EventEffect::CharacterResurrection { character, .. }
                                 if character == participant)
                    });

                    if !is_resurrection {
                        return Err(format!("Dead character {} acts in event", participant));
                    }
                }
            }

            // Apply death/resurrection effects
            for effect in &event.effects {
                match effect {
                    EventEffect::CharacterDeath { character } => {
                        alive_state.insert(*character, false);
                    }
                    EventEffect::CharacterResurrection { character, .. } => {
                        alive_state.insert(*character, true);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
```

**Why this matters**: Prevents ghost dialogue—dead characters speaking without explanation.

## Part 4: Generating Test Cases

### Define Narrative Actions

```rust
pub enum NarrativeAction {
    CreateCharacter { name: String, timeline: TimelineId },
    KillCharacter { character: CharacterId, timeline: TimelineId },
    ResurrectCharacter { character: CharacterId, mechanism: String },
    TradeMemory { memory: MemoryId, from: CharacterId, to: CharacterId },
    BranchTimeline { parent: TimelineId },
    ViolateCausality { timeline: TimelineId, violation: CausalityViolation },
    // ... etc
}
```

### Create Proptest Strategies

```rust
pub fn narrative_action_strategy() -> impl Strategy<Value = NarrativeAction> {
    prop_oneof![
        // Kill character
        (character_id_strategy(), timeline_id_strategy())
            .prop_map(|(character, timeline)| {
                NarrativeAction::KillCharacter { character, timeline }
            }),

        // Resurrect character
        (character_id_strategy(), timeline_id_strategy(), "[A-Z][a-z]{5,15}")
            .prop_map(|(character, timeline, mechanism)| {
                NarrativeAction::ResurrectCharacter {
                    character,
                    timeline,
                    mechanism,
                }
            }),

        // ... more actions
    ]
}
```

### Run Property Tests

```rust
proptest! {
    #[test]
    fn test_random_narrative_sequences(
        actions in prop::collection::vec(narrative_action_strategy(), 10..50)
    ) {
        let mut multiverse = Multiverse::new();

        // Create protagonists
        for name in &PROTAGONIST_NAMES {
            multiverse.create_character(name.to_string(), multiverse.root_timeline);
        }

        // Apply random actions
        for action in &actions {
            apply_narrative_action(&mut multiverse, action);

            // After EVERY action, properties must hold
            if let Err(e) = validate_all_properties(&multiverse) {
                panic!("Property violation after {:?}: {}", action, e);
            }
        }

        prop_assert!(validate_all_properties(&multiverse).is_ok());
    }
}
```

## Part 5: Reading Test Failures

### Example Failure

```
Test failed: Property violation after action
KillCharacter { character: CharacterId(0), timeline: TimelineId(1) }:
Character Char#0 alive status is false but should be true based on timeline events.

minimal failing input: actions = [
    BranchTimeline { parent: TimelineId(0) },
    KillCharacter { character: CharacterId(0), timeline: TimelineId(1) },
]
```

### Interpreting the Failure

1. **Timeline branches**: TimelineId(0) → TimelineId(1)
2. **Character dies in timeline 1**: CharacterId(0) dies in timeline 1
3. **Bug**: Character's `alive` status becomes globally false
4. **Expected**: Character should still be alive in timeline 0

### The Fix

**Before** (buggy):
```rust
pub struct Character {
    pub alive: bool,  // Global alive state - WRONG!
}
```

**After** (correct):
```rust
pub struct Character {
    pub alive_in_timelines: HashMap<TimelineId, bool>,
}
```

Or, track death events in timelines and compute alive status on-demand.

## Part 6: Advanced Scenarios

### Scenario: Memory Trading with Timeline Branches

```rust
proptest! {
    #[test]
    fn test_cross_timeline_memory_trade() {
        // Character A in Timeline 1 has a memory
        // Character B in Timeline 2 acquires it
        // Character B should NOT be able to reference Timeline 1 events
        // UNLESS they have TimelinePerception ability
    }
}
```

### Scenario: Causality Violations

```rust
proptest! {
    #[test]
    fn test_causality_violations_require_mechanisms() {
        // Generate random events
        // Some violate causality
        // Property: All causality violations MUST have a mechanism
        //   (Living Gate, time weapon, reality hack, etc.)
    }
}
```

### Scenario: Relationship Networks

```rust
proptest! {
    #[test]
    fn test_relationship_consistency() {
        // Relationships change via explicit events
        // Within a timeline, relationships must be transitive
        // If A hates B and B hates C, certain dialogue paths should be blocked
    }
}
```

## Part 7: Integration with Game Development

### YarnSpinner Integration (Future)

```yarn
Kim: I can't believe you'd accuse me of this.
<<affects KimRelationship: -30>>
<<requires_acknowledgment: betrayal>>
<<grants_knowledge: kim_distrusts_player>>
```

**Compiler generates**:
```rust
Event {
    effects: vec![
        EventEffect::RelationshipChange {
            character1: player,
            character2: kim,
            new_state: RelationshipState::Distrustful,
        },
        EventEffect::KnowledgeGained {
            character: player,
            flag: "kim_distrusts_player",
        },
    ],
}
```

**Property test validates**:
- Relationship actually changed
- Knowledge flag was set
- Future dialogue references the betrayal

### CI/CD Integration

```yaml
# .github/workflows/test.yml
- name: Run property tests
  run: |
    PROPTEST_CASES=1000 cargo test
    cargo test --release  # Faster, runs more cases
```

### Narrative Design Workflow

1. **Writer writes dialogue** in YarnSpinner
2. **Compiler generates events** with explicit effects
3. **Property tests run** on every commit
4. **If tests fail**: Bug found! Fix narrative or code
5. **If tests pass**: Ship with confidence

## Part 8: Best Practices

### Do: Make State Explicit

✅ **Good**:
```rust
pub struct Character {
    pub knowledge_flags: HashSet<String>,  // Explicit
}
```

❌ **Bad**:
```rust
// Knowledge is implicit in which dialogue nodes were visited
```

### Do: Track Provenance

✅ **Good**:
```rust
pub enum MemoryProvenance {
    Witnessed { character: CharacterId },
    Traded { original_owner: CharacterId, mechanism: String },
    Forged { forger: String },
}
```

❌ **Bad**:
```rust
pub struct Memory {
    pub content: String,  // No provenance tracking
}
```

### Do: Validate Continuously

✅ **Good**:
```rust
for action in &actions {
    apply_action(action);
    validate_all_properties()?;  // Check after EVERY action
}
```

❌ **Bad**:
```rust
for action in &actions {
    apply_action(action);
}
validate_all_properties()?;  // Only check at the end
```

### Do: Write Shrinkable Actions

✅ **Good**:
```rust
prop_oneof![
    timeline_id_strategy().prop_map(|t| Action::BranchTimeline { parent: t }),
    // Proptest can shrink these independently
]
```

❌ **Bad**:
```rust
// Monolithic action that can't be simplified
Action::ComplexMegaAction { /* 20 parameters */ }
```

## Part 9: Measuring Success

### Coverage Metrics

```bash
# How many timeline branches were explored?
# How many character death/resurrection cycles?
# How many memory trades?
# How many causality violations?
```

### Confidence Intervals

```
After 10,000 test runs:
- 0 failures → High confidence
- 1-10 failures → Medium confidence (edge cases)
- 10+ failures → Low confidence (systemic issues)
```

### Shrinking Quality

```
Original failing case: 50 actions
Shrunk case: 3 actions
Shrink ratio: 94% reduction

Better shrinking = easier debugging
```

## Conclusion

Property-based testing for narratives:
- ✅ **Catches edge cases** manual testing would miss
- ✅ **Provides confidence** to write complex stories
- ✅ **Documents invariants** that must always hold
- ✅ **Shrinks failures** to minimal reproductions
- ✅ **Scales** to 13 protagonists, multiple timelines, memory trading, causality violations

Traditional QA testing: "Does THIS scenario work?"
Property testing: "Do ALL scenarios respect these invariants?"

---

**Next Steps**:
1. Run `cargo test` to see property testing in action
2. Read `src/narrative_core.rs` for literate implementation
3. Explore `notes/space_western.md` for full world-building
4. Experiment with adding new properties or actions

**"Causality becomes *optional*—you can choose to live linearly or experience time as a mosaic."**
— The Thirteen Suns, Ending #10
