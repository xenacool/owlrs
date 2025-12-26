# A Conversation on Property Testing for Narrative Validation

**Participants:** Robert Kurvitz (Disco Elysium Lead Designer/Writer), Kate Compton (Procedural Generation Researcher, Tracery creator), Leslie Lamport (Computer Scientist, TLA+ creator), Joe Armstrong (Erlang creator, advocate of property-based testing)

---

**Joe Armstrong:** I've been listening to your discussion about TLA+ and formal specs, and I think you're overcomplicating this. You don't need full formal verification. What you need is property-based testing.

**Leslie:** Property testing? Like QuickCheck?

**Joe:** Exactly. Instead of trying to verify all possible states formally, you write properties that should hold, then generate thousands of random game playthroughs and verify the properties hold for all of them. It's practical, it's fast, and it catches the same bugs you're worried about.

**Kate:** But we were trying to get exhaustive verification. Property testing is still sampling the state space.

**Joe:** Yes, but intelligently. And here's the thing—you can actually run the tests. TLA+ specs are great in theory, but in practice, how many game studios are going to hire formal methods experts? With property testing in Rust using proptest, you're writing code that looks like normal tests. Any programmer can do it.

**Robert:** Show me what that looks like for narrative.

**Joe:** Alright. In Rust with proptest and YarnSpinner, you'd write something like:

```rust
proptest! {
    #[test]
    fn kim_relationship_stays_coherent(
        actions in vec(dialogue_action_strategy(), 1..100)
    ) {
        let mut game_state = GameState::new();

        for action in actions {
            game_state.execute_dialogue(action);

            // Property: If Kim's relationship is Distrustful or Hostile,
            // dialogue must acknowledge betrayal
            if game_state.kim_relationship <= RelationshipState::Distrustful {
                prop_assert!(
                    game_state.dialogue_flags.contains("kim_betrayal_acknowledged"),
                    "Kim distrusts player but betrayal was never acknowledged"
                );
            }

            // Property: Dead characters can't speak
            if game_state.is_dead("kim") {
                prop_assert!(
                    !game_state.can_speak_with("kim"),
                    "Kim is dead but dialogue is still available"
                );
            }
        }
    }
}
```

You run this, and proptest generates random sequences of dialogue choices, runs them through your game logic, and checks that your properties hold.

**Leslie:** That's clever. You're not proving correctness, but you're finding violations quickly.

**Joe:** And when it finds a violation, it *shrinks* the failing case. It gives you the minimal sequence of actions that breaks the property. Just like your TLA+ counterexample, but you get it in seconds instead of hours of model checking.

**Kate:** What about coverage? How do you know you've tested enough?

**Joe:** You don't get mathematical proof of coverage, but in practice, after thousands of randomized playthroughs, you've hit most of the interesting edge cases. And you can guide the generator—tell it to favor certain kinds of actions, ensure it explores paths you care about.

**Robert:** This feels more feasible for a game studio. We already have test infrastructure.

**Joe:** Exactly. And here's where it gets interesting for your narrative problem: you can use property testing to explore narrative *spaces*. You're not just validating—you're generating.

**Kate:** How so?

**Joe:** Write properties that define valid narratives, then use proptest to *generate* valid narrative sequences. Filter out the boring ones, keep the interesting ones. You've got procedural narrative generation with built-in coherence guarantees.

**Leslie:** So property testing becomes both validator and generator.

**Joe:** Precisely. And for time-travel narratives, multiverses, all that Philip K. Dick stuff—property testing is perfect because you're explicitly checking temporal properties across branching timelines.

**Robert:** Wait, explain that. How does property testing handle time travel?

**Kate:** Oh, I see where he's going. In a time-travel narrative, you have causality violations, branching timelines, parallel universes. These are exactly the kinds of complex state dependencies that are hard to reason about.

**Joe:** Right. Let me give you an example. Say you're building a narrative where the player can travel back in time and change events. You need to ensure that changes propagate correctly. Here's a property you might check:

```rust
proptest! {
    #[test]
    fn time_travel_preserves_causality(
        initial_timeline in timeline_strategy(),
        time_travel_action in time_travel_strategy()
    ) {
        let mut universe = Universe::new(initial_timeline);

        // Player travels back and changes something
        universe.travel_to(time_travel_action.target_time);
        universe.execute_action(time_travel_action.action);

        // Property: Effects of the change must propagate forward
        // to the "present" the player came from
        let original_time = time_travel_action.origin_time;
        universe.advance_to(original_time);

        prop_assert!(
            universe.timeline_has_diverged(),
            "Player changed the past but present is unchanged"
        );

        // Property: Characters must remember the original timeline
        // OR the new timeline, never both (unless they're time-aware)
        for character in universe.characters() {
            let memories = universe.get_memories(character);
            let original_count = memories.from_timeline(TimelineId::Original).count();
            let new_count = memories.from_timeline(TimelineId::Current).count();

            if !character.is_time_aware() {
                prop_assert!(
                    original_count == 0 || new_count == 0,
                    "Non-time-aware character has memories from multiple timelines"
                );
            }
        }
    }
}
```

**Robert:** So you're generating random time-travel scenarios and checking that your causality rules hold?

**Joe:** Exactly. And proptest will find the edge cases—like when the player kills their past self, or creates a grandfather paradox, or whatever. You specify *what* should happen in those cases as properties, and the test framework finds scenarios that violate them.

**Leslie:** This is actually quite elegant. You're encoding the same temporal logic I'd write in TLA+, but as executable properties that run fast.

**Kate:** What about multiverses? Like in C.L. Moore's stories where parallel timelines coexist?

**Robert:** Or Asimov's "The End of Eternity" where Reality changes get propagated through time.

**Joe:** Perfect examples. Let's say you have a multiverse where player actions create branches. You might have properties like:

**Property 1: Timeline Divergence Consistency**

```rust
proptest! {
    #[test]
    fn timeline_branches_are_consistent(
        actions in vec(player_action_strategy(), 5..50)
    ) {
        let mut multiverse = Multiverse::new();
        let original_timeline = multiverse.create_timeline();

        multiverse.set_active_timeline(original_timeline);

        for action in actions {
            multiverse.execute_action(action);

            if action.creates_branch() {
                // Property: Branching creates a new timeline that shares
                // history up to the branch point but diverges after
                let new_timeline = multiverse.latest_timeline();
                let branch_point = action.timestamp();

                prop_assert!(
                    multiverse.histories_match_until(
                        original_timeline,
                        new_timeline,
                        branch_point
                    ),
                    "Branched timeline diverges before branch point"
                );

                prop_assert!(
                    multiverse.histories_diverge_after(
                        original_timeline,
                        new_timeline,
                        branch_point
                    ),
                    "Branched timeline doesn't actually diverge"
                );
            }
        }
    }
}
```

**Kate:** That's checking that branches behave correctly—they share past history but differ in the future.

**Joe:** Right. Here's another one, inspired by PKD's "Man in the High Castle"—what if characters can perceive alternate timelines?

**Property 2: Cross-Timeline Perception**

```rust
proptest! {
    #[test]
    fn characters_perceive_correct_timelines(
        scenario in multiverse_scenario_strategy()
    ) {
        let mut multiverse = Multiverse::from_scenario(scenario);

        for character in multiverse.characters() {
            let perceived_timelines = character.perceivable_timelines();
            let character_timeline = character.native_timeline();

            // Property: Characters always perceive their native timeline
            prop_assert!(
                perceived_timelines.contains(&character_timeline),
                "Character can't perceive their own timeline"
            );

            // Property: Cross-timeline perception requires special ability
            for timeline in perceived_timelines {
                if timeline != character_timeline {
                    prop_assert!(
                        character.has_ability(Ability::TimelinePerception) ||
                        character.has_item(Item::InteruniversalDevice),
                        "Character perceives alternate timeline without explanation"
                    );
                }
            }

            // Property: Dialogue must reflect cross-timeline knowledge
            if perceived_timelines.len() > 1 {
                let dialogue = multiverse.get_available_dialogue(character);
                prop_assert!(
                    dialogue.any(|d| d.references_alternate_timelines()),
                    "Character perceives multiple timelines but dialogue doesn't reflect it"
                );
            }
        }
    }
}
```

**Robert:** Oh, that's good. That catches cases where we give a character knowledge from another timeline but forget to have them mention it, or vice versa.

**Leslie:** You're encoding invariants as runtime checks. The difference from TLA+ is you're sampling, not proving exhaustively.

**Joe:** But for game development, sampling is enough. You run these tests on every commit. Each test run explores thousands of narrative paths. Over time, you've explored millions of combinations. In practice, you catch the bugs.

**Kate:** What's the third property? We need something about causality propagation.

**Joe:** Right. Let's do one inspired by Asimov's "End of Eternity"—when you change the past, effects ripple forward through time:

**Property 3: Causal Propagation Completeness**

```rust
proptest! {
    #[test]
    fn past_changes_propagate_completely(
        original_events in timeline_events_strategy(20..100),
        intervention in past_intervention_strategy()
    ) {
        // Create timeline with events
        let mut timeline = Timeline::from_events(original_events);
        let intervention_time = intervention.time;

        // Record what exists after intervention point
        let future_events = timeline.events_after(intervention_time);
        let affected_entities = future_events.iter()
            .flat_map(|e| e.involved_entities())
            .collect::<HashSet<_>>();

        // Apply intervention in the past
        timeline.apply_intervention(intervention);
        timeline.recompute_from(intervention_time);

        // Property: All entities that existed in the future must either:
        // 1. Still exist (possibly modified), OR
        // 2. Be explicitly marked as erased/never-existed
        for entity in affected_entities {
            let still_exists = timeline.entity_exists(entity);
            let marked_erased = timeline.is_erased(entity);

            prop_assert!(
                still_exists || marked_erased,
                "Entity {} existed in original future but is neither present \
                 nor marked as erased after past intervention",
                entity
            );

            // Property: If entity still exists but was affected,
            // they must have dialogue acknowledging timeline change
            if still_exists && timeline.entity_was_affected(entity) {
                let dialogue = timeline.get_dialogue_state(entity);
                prop_assert!(
                    dialogue.acknowledges_timeline_change ||
                    !entity.is_time_aware(),
                    "Time-aware entity affected by change but doesn't acknowledge it"
                );
            }
        }

        // Property: Causal chains must be complete
        // If A caused B, and A is removed, B must also be removed or re-explained
        let original_causes = timeline.causal_graph_before_intervention();
        let new_causes = timeline.causal_graph_after_intervention();

        for (effect, causes) in original_causes.iter() {
            if timeline.entity_exists(*effect) {
                let effect_still_caused = new_causes.get(effect)
                    .map(|c| !c.is_empty())
                    .unwrap_or(false);

                prop_assert!(
                    effect_still_caused,
                    "Effect {} still exists but its causes were removed by intervention",
                    effect
                );
            }
        }
    }
}
```

**Robert:** That's... exactly what we needed. It checks that when you change history, everything that depended on the old history either gets updated or disappears.

**Kate:** And the "causal graph" part is brilliant—you're tracking dependencies between events and ensuring they stay consistent.

**Joe:** The beauty is, you write this once, and every test run explores different scenarios. Different intervention times, different events, different causal structures. Proptest generates them all.

**Leslie:** I'm impressed. This is more practical than what I was proposing. You lose exhaustiveness but gain implementability.

**Joe:** And you can combine approaches. Use property testing during development for fast feedback. Use TLA+ for critical subsystems where you need proof. But for most narrative coherence checking, property testing is sufficient and way more accessible.

**Robert:** What about authoring? How do writers work with this?

**Joe:** Same as Leslie suggested—writers write in YarnSpinner with metadata. But instead of compiling to TLA+ specs, you compile to property tests. The properties are defined by narrative designers once, then automatically checked against all new content.

**Kate:** So writers write:

```
Kim: I can't believe you'd accuse me of this.
<<affects KimRelationship: -30>>
<<requires_acknowledgment: betrayal>>
```

And your tooling automatically includes that in the property test suite?

**Joe:** Exactly. Every new dialogue node becomes part of the input space that proptest explores. The properties check that relationships stay coherent, that acknowledgment flags are set, that dead characters don't speak—all the invariants you care about.

**Leslie:** The shrinking behavior is particularly valuable. When a test fails, you don't just get "some 50-action sequence broke the game," you get "this specific 3-action sequence is the minimal reproduction."

**Robert:** That's a better bug report than playtesting gives us. Playtesting gives us "something felt wrong in Act 3," this gives us "dialogue 47 → dialogue 203 → dialogue 412 breaks Kim's state."

**Joe:** And you can run it continuously. On every commit, generate 10,000 random playthroughs, check all properties. Takes minutes, catches bugs immediately.

**Kate:** What about the generative side? You mentioned using this for procedural narrative?

**Joe:** Write your properties to define what makes a *valid* narrative. Then generate random narrative sequences, filter to those that pass all properties, and rank them by quality metrics. The properties guarantee coherence, the ranking selects for interestingness.

**Robert:** So coherence is a hard constraint checked by properties, and dramatic quality is a soft constraint scored separately?

**Joe:** Precisely. You get the best of both worlds—formal guarantees of coherence via properties, and creative flexibility in what constitutes "good" drama.

**Leslie:** This is remarkably practical. The theoretical basis is sound—you're doing statistical model checking. Not as strong as formal proof, but sufficient for this domain.

**Kate:** And it extends naturally to wild narrative structures—time loops, multiverses, branching timelines, memory wipes, all of that. You just write properties that express what should be true, and test them.

**Robert:** Give me one more example—something really weird. Like a time loop where the player retains memories across loops but the world resets.

**Joe:** Like "Edge of Tomorrow" or "Groundhog Day"? Here's a property:

```rust
proptest! {
    #[test]
    fn time_loop_memory_consistency(
        loop_actions in vec(vec(action_strategy(), 1..20), 3..10)
    ) {
        let mut game = GameState::new();
        let mut player_knowledge = KnowledgeSet::new();

        for (loop_num, actions) in loop_actions.iter().enumerate() {
            // Reset world but preserve player knowledge
            game.reset_world_to_loop_start();
            game.set_player_knowledge(player_knowledge.clone());

            for action in actions {
                game.execute(action);

                // Player learns something new this loop
                if action.grants_knowledge() {
                    player_knowledge.insert(action.knowledge());
                }
            }

            // Property: Player can reference anything learned in previous loops
            let available_dialogue = game.get_player_dialogue_options();
            for knowledge in player_knowledge.iter() {
                let knowledge_referenced = available_dialogue.iter()
                    .any(|d| d.can_reference(knowledge));

                prop_assert!(
                    knowledge_referenced,
                    "Player has knowledge from loop {} but can't reference it in loop {}",
                    knowledge.acquired_loop, loop_num
                );
            }

            // Property: NPCs don't remember previous loops (unless time-aware)
            for npc in game.npcs() {
                if !npc.is_time_aware() {
                    prop_assert!(
                        npc.memory.loop_num() == loop_num,
                        "Non-time-aware NPC {} has memories from previous loops",
                        npc.name
                    );
                }
            }
        }
    }
}
```

**Robert:** That's perfect. It checks that knowledge accumulates correctly for the player but resets for NPCs.

**Kate:** And you could extend it—check that NPCs react to the player having "impossible" knowledge, check that time-aware NPCs notice the loop, all of that.

**Joe:** The key insight is: complex narrative structures are just state machines with properties. Write the properties clearly, test them exhaustively with random generation, and you'll catch the bugs.

**Leslie:** I concede this is more practical for game development than TLA+. Though I still think TLA+ has value for reasoning about the design before you implement it.

**Joe:** Fair. Use TLA+ for design, use property testing for implementation. But the property tests should be sufficient for most studios.

**Robert:** Alright, I'm convinced. What's the implementation path?

**Joe:**
1. Identify your narrative invariants—the rules that must always hold
2. Express them as properties in Rust using proptest
3. Write generators that produce random dialogue sequences
4. Run the properties against thousands of generated sequences
5. When they fail, fix the narrative or refine the property
6. Integrate with CI/CD—every commit runs the tests

**Kate:** And we use literate programming to document the properties alongside the narrative rules?

**Joe:** Absolutely. Each property is a human-readable specification of a narrative rule. The code is the documentation.

**Robert:** This feels achievable. Time travel, multiverses, all the PKD/Asimov weirdness—it's all just properties to check.

**Leslie:** The formalization still matters. Writing these properties forces you to think clearly about what your narrative rules are.

**Joe:** Agreed. The act of writing properties is design. The testing is validation. Both are valuable.

**Kate:** Formal methods for narrative games, made practical through property testing.

**Joe:** Welcome to the 21st century. We have the tools. Now use them.

---

*End of conversation*
