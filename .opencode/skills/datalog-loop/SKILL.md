---
name: datalog-loop
description: PLACEHOLDER. Iterative loop for the verdigris-datalog experiment — pick the next legality question, express it in Datalog, check it against the verdigris reference (oracle), commit, report. User-invoked only (/datalog-loop [tier overrides] [notes]).
disable-model-invocation: true
---

# Datalog loop (placeholder)

Not ready to run. Phases, tiers and acceptance checks are written after the two pending fixes on the
reference branch (per-element waivers, Filter-based choice candidates) and the Level 2 vs Level 3 decision.

Planned shape (to be confirmed):

1. **Preflight** — load `opencode-orchestrator` (+ `orchestrator-core`); baseline; work dir
   `.local-run/orchestrator/<date>-datalog-<topic>/`; read `DATALOG.md`.
2. **Pick** — next question from a checklist (cast, play, attack, block, targets, waivers, why-not, ...).
3. **Express** — relations + rules for it (skill `verdigris-datalog`).
4. **Check** — oracle: Datalog answers == `rules::Rules::legality` / `LegalActions` on the existing tests.
5. **Commit + report** — on the current branch only.
