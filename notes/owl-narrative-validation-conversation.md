# A Conversation on Formal Methods for Narrative Validation

**Participants:** Robert Kurvitz (Disco Elysium Lead Designer/Writer), Kate Compton (Procedural Generation Researcher, Tracery creator), Leslie Lamport (Computer Scientist, TLA+ creator, Turing Award winner)

---

**Robert Kurvitz:** The fundamental problem is this: in Disco Elysium, we had 300,000 words, dozens of skills with voices, political alignments, clothing affecting stats, branching dialogue. We tracked coherency through spreadsheets and exhaustive reading. It barely worked. Now imagine 10x that size, or procedurally varying content. How do we validate that the narrative stays coherent across millions of possible player paths?

**Kate Compton:** State space explosion. You have hundreds of variables—skill levels, the Thought Cabinet, quest states, political alignments, items, NPC relationship values. Each combination opens different narrative branches. The combinatorics are brutal.

**Robert:** And it's not just *reachability*—it's *semantic coherency*. Can you be a communist who worships the free market? Maybe, but the dialogue must acknowledge that tension. Can you intimidate someone and have them trust you later without any narrative recognition of the contradiction?

**Leslie Lamport:** This is a specification and verification problem. You need to formally specify what "coherent" means for your narrative system, then verify that all reachable states satisfy that specification. I've spent decades on exactly this problem, just not for narratives.

**Kate:** I was thinking OWL reasoning—ontological consistency checking. You model narrative facts as an ontology and use a reasoner to catch contradictions.

**Leslie:** OWL is one approach, but let me suggest something more practical for this scale: TLA+. It's designed specifically for specifying and verifying systems with complex state spaces. You describe the system's behavior as state transitions, specify invariants that must hold, and use a model checker to verify them across all possible execution paths.

**Robert:** How would that work for narrative?

**Leslie:** Think of your game as a state machine. Each state is a complete snapshot: every variable value, every NPC relationship, every flag. Each player action is a transition to a new state. You write TLA+ specifications that describe:
1. The initial state
2. The rules for state transitions (what actions are possible in what states)
3. Invariants that must always be true
4. Temporal properties (if X happens, eventually Y must happen)

Then TLC, the model checker, explores all possible paths through your state space and verifies your properties hold.

**Kate:** But the state space is enormous. How do you model check that?

**Leslie:** You don't check the entire space at once. You partition. Check subsystems independently. Use symmetry reduction. Specify the system at the right level of abstraction. The art is in the specification—what to include, what to abstract away.

**Robert:** Give me a concrete example.

**Leslie:** Suppose you have an NPC, Kim Kitsuragi. His relationship with the player has states: Neutral, Trusting, Distrustful, Hostile. You have dialogue actions that can transition between these states. In TLA+, you'd write something like:

```
KimRelationship ∈ {Neutral, Trusting, Distrustful, Hostile}

AccuseKimOfCorruption ==
  ∧ KimRelationship ∈ {Neutral, Trusting}
  ∧ KimRelationship' = Distrustful
  ∧ KimDialogueAcknowledgesBetrayal' = TRUE

DefendKimPublicly ==
  ∧ KimRelationship ∈ {Neutral, Distrustful}
  ∧ KimRelationship' = Trusting
  ∧ KimDialogueAcknowledgesLoyalty' = TRUE
```

Then you specify invariants like:
```
TypeInvariant ==
  ∧ KimRelationship ∈ {Neutral, Trusting, Distrustful, Hostile}
  ∧ KimDialogueAcknowledgesBetrayal ⇒ (KimRelationship ∈ {Distrustful, Hostile})
```

The model checker verifies that in every reachable state, if the betrayal flag is set, Kim's relationship reflects that.

**Kate:** So it's like unit testing, but for all possible game states at once.

**Leslie:** Exactly. And when it finds a violation, it gives you a counterexample—a specific sequence of player actions that leads to the inconsistency. That's your bug report.

**Robert:** That's gold for debugging. "Here's the exact path that breaks coherency."

**Kate:** But how do you handle the authoring problem? We can't have writers learning TLA+.

**Leslie:** You don't expose TLA+ to writers. You build a pipeline. Writers work in YarnSpinner or whatever dialogue system they're comfortable with. They add metadata annotations:

```
Player: I think you're corrupt, Kim.
<<affects KimRelationship: Neutral|Trusting -> Distrustful>>
<<requires_acknowledgment: betrayal>>
```

Your tooling compiles this into TLA+ specifications automatically. Writers write narrative, the tool generates formal specifications, TLC verifies them.

**Robert:** What about literate programming? Could we document the specifications alongside the narrative?

**Leslie:** Absolutely. Literate programming is perfect for this. You write a document that's both human-readable explanation and executable specification. You might have:

```
# Kim Kitsuragi Relationship System

Kim's trust is fragile. Once betrayed, he becomes skeptical of the player.
This is a core narrative principle: trust is harder to build than to destroy.

In TLA+:
<<KimRelationship_Spec.tla>>
  ...specification here...
<<>>

This ensures that if KimRelationship reaches Distrustful through betrayal,
no dialogue can transition back to Trusting without a major redemptive act.
```

The tooling extracts the TLA+ code, verifies it, and maintains the narrative documentation. Writers see the human explanation, engineers see the formal spec, both are in sync.

**Kate:** What if the state space is still too large? Even with partitioning?

**Leslie:** Here's where it gets interesting. You don't need one monolithic TLA+ spec. You brute-force enumerate the major narrative paths—the "spine" of possible playthroughs—and generate a separate TLA+ spec for each one. Each spec is tractable because it's a specific path with variations, not the entire possibility space.

**Robert:** So instead of checking all 10^20 possible states, you check 1000 specs of 10^8 states each?

**Leslie:** Precisely. You're partitioning temporally, not just spatially. Each playthrough type—"detective who trusts Kim," "detective who betrays Kim," "detective who ignores Kim"—gets its own specification. You verify each independently. The combinatorial explosion is contained.

**Kate:** And you could use procedural generation to create those specs. Enumerate the major decision points, generate the cross-product, output TLA+ specs programmatically.

**Leslie:** Yes. Your authoring tool becomes a spec generator. Writers define decision trees and constraints, the tool enumerates narrative paths, generates TLA+ specifications for each path, runs model checking in parallel, and reports any inconsistencies back to the writers in human terms.

**Robert:** What about things like... ideological coherency? If the player espouses communist views in Act 1 and fascist views in Act 2, some NPCs should react to that shift.

**Leslie:** That's a temporal property. In TLA+:

```
IdeologicalCoherency ==
  (PlayerIdeology = Communist) ~> (NPCDialogueReflectsCommunism = TRUE)

IdeologicalShiftRecognized ==
  ((PlayerIdeology = Communist) ∧ ◊(PlayerIdeology = Fascist))
    ⇒ ◊(NPCDialogueAcknowledgesShift = TRUE)
```

The first says: if the player is communist, eventually dialogue must reflect that. The second says: if ideology shifts from communist to fascist, eventually an NPC must acknowledge the shift.

**Kate:** You're using temporal logic to capture narrative causality.

**Leslie:** That's exactly what temporal logic is for. It captures properties that unfold over time. Narrative is fundamentally temporal. TLA+ gives you the operators to express "eventually," "always," "until," "leads to."

**Robert:** Can we express more complex narrative properties? Like, "if you complete quest A before quest B, character C's arc must resolve differently"?

**Leslie:** Yes:

```
QuestOrderingMatters ==
  (QuestAComplete ∧ ¬QuestBComplete) ~> (CharacterCArcState = PathX)
  ∧
  (QuestBComplete ∧ ¬QuestAComplete) ~> (CharacterCArcState = PathY)
```

You specify the causal relationships, and the model checker verifies they hold.

**Kate:** This is starting to feel practical. What about the OWL reasoning we were discussing? Does that still have a place?

**Leslie:** OWL and TLA+ serve different purposes. OWL is great for ontological classification—"this thing is-a kind of that thing." TLA+ is great for behavioral verification—"this system behaves correctly over time." You could use OWL to define your narrative ontology—character types, event types, relationship types—and then compile that ontology into TLA+ type constraints.

**Robert:** So OWL defines the vocabulary, TLA+ defines the grammar of how that vocabulary can be used over time?

**Leslie:** Good analogy. OWL says "these are the kinds of things that exist." TLA+ says "here's how those things can change and what must remain true as they change."

**Kate:** You could also skip OWL entirely and just use TLA+. It depends on whether you need the inferencing capabilities of OWL. If you're just checking properties you've explicitly specified, TLA+ is simpler and more direct.

**Leslie:** Agreed. Start with TLA+ alone. If you find you need rich ontological reasoning—"infer that this character must hate the player because of these indirect relationships"—then add OWL. But for validating explicitly specified narrative constraints, TLA+ is sufficient and more efficient.

**Robert:** What about the tooling? What would the full pipeline look like?

**Leslie:** Here's what I'd build:

1. **Narrative Authoring Layer**: Writers use YarnSpinner/Ink/whatever with metadata annotations describing state changes and constraints.

2. **Narrative Compiler**: Parses dialogue files, extracts state variables and transitions, generates TLA+ specifications. Possibly generates multiple specs for different narrative paths.

3. **Specification Partitioner**: Analyzes the generated specs, breaks them into manageable chunks based on narrative structure (chapters, character arcs, locations), identifies dependencies.

4. **Model Checking Layer**: Runs TLC in parallel on each spec partition. Uses a compute cluster if the game is large enough to warrant it.

5. **Violation Reporter**: When TLC finds a violation, translates the counterexample back into narrative terms: "If player does X in dialogue 47, then Y in dialogue 203, NPC Z's state becomes inconsistent at dialogue 412."

6. **Literate Documentation Generator**: Produces human-readable documents that explain the specifications, include the formal TLA+ code, and link to the relevant dialogue files.

**Kate:** That's a full toolchain. How feasible is this?

**Leslie:** Every piece exists. TLA+ and TLC are mature and free. Parsers for dialogue systems are straightforward. The novel part is the narrative-to-TLA+ compiler, but that's just code generation based on templates. The hardest part is figuring out the right level of abstraction for specifications.

**Robert:** What do you mean?

**Leslie:** If you model every single variable at full precision, you'll have billions of states. If you abstract too much, you'll miss real bugs. The art is choosing what to model precisely and what to abstract. For example, do you model Kim's relationship as five discrete states or as an integer from 0-100? Discrete is easier to verify but less expressive.

**Kate:** This is the same tension as in procedural generation. Too much structure and you lose expressiveness. Too little and you lose coherence.

**Leslie:** Exactly. But here's the advantage of literate programming with TLA+: you can have multiple specifications at different abstraction levels. A high-level spec that models Kim's relationship as five states, and a lower-level spec that models it as a numeric value with thresholds. You verify different properties at different levels.

**Robert:** Give me a concrete workflow. Let's say a writer adds a new quest that involves Kim.

**Leslie:** Here's the flow:

1. Writer adds dialogue in YarnSpinner: "Player accuses Kim of covering up evidence."

2. Writer adds metadata: `<<affects KimRelationship: -20>> <<requires_acknowledgment: kim_betrayed>>`

3. On save, the tooling runs: extracts the new dialogue, generates/updates the TLA+ spec for the "Kim relationship" subsystem, runs TLC on that spec.

4. TLC finds a violation: "In a reachable state, KimRelationship < -50 but dialogue file kim_act3_reconciliation.yarn still fires, which requires KimRelationship > 0."

5. Tool reports to writer: "Your new dialogue creates a path where Kim can be reconciled even after severe betrayal, but no dialogue acknowledges this. Either add a gate condition or write new reconciliation dialogue that accounts for betrayal."

6. Writer fixes it. Tooling re-verifies. Success.

This happens continuously during development, catching bugs as they're introduced.

**Kate:** It's like continuous integration for narrative coherence.

**Leslie:** Exactly. CI/CD for narrative. Every commit runs the specs. Any violation blocks the merge.

**Robert:** What about performance? How long do these checks take?

**Leslie:** Depends on the spec size. A well-partitioned spec might verify in seconds to minutes. A complex spec with deep state might take hours. But you can parallelize—run different specs on different machines. And you only re-verify the specs that are affected by changed dialogue files.

**Kate:** Incremental verification.

**Leslie:** Yes. If you edit dialogue in Chapter 3, you don't need to re-verify Chapter 7 unless they share state.

**Robert:** This feels achievable. What's the MVP?

**Leslie:** Start small. Pick one narrative subsystem—say, Kim's relationship system. Hand-write a TLA+ spec for it. Write a few invariants. Verify them with TLC. Once you see it working, build the compiler that generates specs from dialogue files. Then gradually expand to more subsystems.

**Kate:** And the literate programming part?

**Leslie:** From day one. Every spec should be a literate document that explains the narrative logic in plain English and includes the formal TLA+ code. This serves as both documentation for the writers and specification for the engineers. It's the shared language.

**Robert:** Could we generate the English from the TLA+ or vice versa?

**Leslie:** Both directions are possible but imperfect. TLA+ to English is easier—you can have templates that describe what each operator means. English to TLA+ is harder—it's an AI problem. I'd start with human-written literate specs and build tooling to keep them in sync with the dialogue files.

**Kate:** What about expressiveness? Can TLA+ capture all the narrative properties we care about?

**Leslie:** It can capture any property you can state precisely. The question is whether you *can* state your narrative property precisely. Things like "this scene feels emotionally resonant" can't be formalized. But "if the player kills character X, character Y's dialogue must reference it" can be.

**Robert:** So we're formalizing the structural and causal properties, not the aesthetic ones.

**Leslie:** Correct. TLA+ ensures your narrative is *coherent*. It doesn't ensure it's *good*. That's still the writer's job.

**Kate:** But coherence at scale is the bottleneck. If you can guarantee coherence mechanically, writers can focus on making it good.

**Robert:** That's the dream. Scale without chaos.

**Leslie:** One more thing to consider: you can use TLA+ not just for verification but for exploration. You can write a spec that's intentionally underspecified—leaves some decisions open—and use TLC to enumerate all the valid possibilities. This could be a tool for procedural generation.

**Kate:** Oh that's interesting. The spec defines the constraints, TLC enumerates solutions, and you select from those solutions based on quality metrics.

**Leslie:** Exactly. Constrained procedural generation. The constraints are hard (verified by TLA+), the selection is soft (scored by your drama metrics).

**Robert:** So TLA+ becomes both the guardrails and the generator.

**Leslie:** It's a tool for thinking clearly about systems. Narrative is a system. TLA+ forces you to think precisely about what you want and what you're guaranteeing. That clarity is valuable even before you run a single model check.

**Kate:** The formalization is the point, not just the verification.

**Leslie:** Precisely. Writing a spec forces you to confront ambiguities in your design. "Wait, what *should* happen if the player does X after Y?" If you can't answer that clearly enough to write a spec, your writers won't be able to implement it coherently either.

**Robert:** So the process of writing specs is itself a design tool.

**Leslie:** Always has been. That's why I've spent 40 years advocating for formal methods. Not because we need mathematical proofs, but because the act of being precise makes us better designers.

**Kate:** Alright, I'm convinced. Where do we start?

**Robert:** I'll identify the core narrative subsystems in Disco Elysium that were hardest to keep coherent. Kim's relationship, the player's political alignment, the thought cabinet, maybe three more.

**Leslie:** I'll draft example TLA+ specs for each. We'll verify them by hand against known playthrough data.

**Kate:** I'll prototype the dialogue-to-spec compiler. We'll need to design the metadata format for writers to annotate dialogue.

**Robert:** And we'll document everything literately—every spec is a human-readable design document.

**Leslie:** When you have something working, even at small scale, this could change how narrative games are built. The industry needs this.

**Kate:** Formal methods for narrative. Who would have thought?

**Leslie:** People have been telling stories for millennia. It's about time we had tools to help us tell them coherently at scale.

---

*End of conversation*
