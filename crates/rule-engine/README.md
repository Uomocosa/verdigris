# verdigris-rule-engine

**Goal: enforce the MTG Comprehensive Rules headlessly: each rule is one Bevy system in `__rules__/`, and every rule is proven by `MockGame` tests.**

One rule file per Comprehensive Rule, with the CR text quoted in its system's doc comment.
The engine reads world state and writes messages, choices and components.
