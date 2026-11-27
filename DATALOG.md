# verdigris-datalog — experiment branch

Status: **placeholder** (2026-11-27). Specifics are decided after two fixes land on
`refactor/keywords-as-abilities` (per-element waivers, Filter-based choice candidates).

## What this branch is for

Try "rules as data" (Datalog) for the verdigris rule engine and decide what to keep.

- Level 2: a Datalog layer answers legality only (can cast / play / attack / block, legal targets,
  restrictions, "as though", why-not), fed with facts from the ECS. Bevy systems still do everything.
- Level 3: card text and rules become Datalog rules loaded as data; the ECS is the fact store; possibly
  incremental evaluation. Probably the direction to aim for (open question).

## Reference

`../verdigris` (branch `refactor/keywords-as-abilities`) is the reference implementation. Its test suite is
the oracle: a Datalog answer must agree with `rules::Rules::legality` / `LegalActions` on every existing test
state (see the oracle helper `MockGame::assert_legal_actions_agree`).

## Branch policy

The user keeps this branch up to date with the reference branch and decides what gets kept.
Work here never touches `../verdigris`.

## Where things live

- Loop / orchestration skill: `.opencode/skills/datalog-loop/SKILL.md` (placeholder)
- Datalog knowledge skill: `.opencode/skills/verdigris-datalog/SKILL.md` (placeholder)
- Work dirs: `.local-run/orchestrator/<date>-datalog-<topic>/` (gitignored)
