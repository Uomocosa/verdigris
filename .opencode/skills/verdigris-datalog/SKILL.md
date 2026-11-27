---
name: verdigris-datalog
description: PLACEHOLDER. How verdigris legality maps to Datalog — facts from the ECS, relations, rules for restrictions/as-though/allow, why-not queries, and the oracle against the reference engine. Load before writing Datalog code in verdigris-datalog.
---

# verdigris → Datalog (placeholder)

To be written after the design discussion. Will cover:

- Fact extraction: zones, controller/owner, characteristics (post-layer), turn/step/priority, counters.
- Mapping: `rules::Condition` atoms -> relations; `Restriction` -> `cant_*` rules; `Change::AsThough`/`Allow`
  -> extra derivations; `.unwaivable()` -> not overridable; list args -> per-element facts.
- Queries: `can_<event>(...)`, `why_not_<event>(..., restriction_id)`, candidates for choices.
- Engine choice (Level 2: `ascent` crate; Level 3: runtime engine) — undecided.
- Oracle: compare with `rules::Rules::legality` and `actions::LegalActions` from `../verdigris`.
