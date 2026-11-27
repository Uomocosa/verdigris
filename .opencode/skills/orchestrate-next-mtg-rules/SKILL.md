---
name: orchestrate-next-mtg-rules
description: Daily verdigris loop — fan out FREE_MODEL proposers over /propose-next-mtg-rules, rank, pick what fits today, implement each with workers, gate, commit, push on the current branch, report. User-invoked only (/orchestrate-next-mtg-rules [tier overrides] [extra notes]).
disable-model-invocation: true
---

# Orchestrate the next MTG rules

**First load the `opencode-orchestrator` skill** (which loads `orchestrator-core`) and follow
it. This file only says WHAT the phases are. Model ids live only in orchestrator-core §0;
overrides in the request (e.g. "use only efficient models", `implement=free`, `--free <id>`)
follow orchestrator-core §0 "Overrides".

Request: $ARGUMENTS

Run on your own from start to report. You and the workers may commit and push as you see fit,
always on the current branch (orchestrator-core: never create, switch or merge branches).

## Default tiers

| Phase | Tier | Parallel |
|---|---|---|
| propose | FREE_MODEL, `--agent plan`, prompt `/propose-next-mtg-rules <lane> --out <work dir>` | 5 lanes: `1xx-3xx`, `4xx-5xx`, `6xx`, `7xx`, `partial` (rules already partly implemented) |
| rank | you, the orchestrator | - |
| implement | EFFICIENT_MODEL (FREE_MODEL also allowed) | one editing worker at a time in the repo |
| escalate | SMART_MODEL, read-only diagnosis after the first failure (orchestrator-core escalation) | - |

## Phases

1. **Preflight.** Baseline commit and branch, models probed, quota OK, shared work dir
   `.local-run/orchestrator/<YYYY-MM-DD-HHMM>-propose-next-mtg-rules/`, `worker-env.md`,
   `plan.md` with the resolved tiers. A dirty tree: work out whose changes they are; commit
   or stash them as you judge, and say so in the report.
2. **Propose.** Launch the 5 lanes in parallel; each writes `<lane>.md`.
3. **Rank.** Build one table: CR, title, worker score, lane, decision (IMPLEMENT /
   DEFERRED / CANNOT TODAY / DROPPED) and the reason. Check the claims yourself before you
   rank: the card exists and is real (drop proposals with made-up cards), the proposed test is
   really red, no rule file or system already covers it. Write the table to `report.md`.
4. **Implement.** Write one brief per IMPLEMENT item (`brief-<X>.md`), one worker, one commit
   each, fast gate per item. Every brief tells the worker to read
   `.opencode/skills/verdigris-rules-style/SKILL.md` and its `references/CHANGEABLE_RULES.md` first. Every `worker-env.md` says: "Tests: red first, by adding a step or assert to an existing test. A new test needs a reason in the report." Log hash, tier + model, minutes in `plan.md`. Re-rank between items
   if an implementation changes what is possible (a DEFERRED item may become doable).
   A brief that adds, removes or reshapes an event field lists EVERY construction site
   (`rg -n '<Event> \{' crates/`) and says where each gets the value, including producers that
   are not events (state-based actions, 608.2 resolution, steps: they use their rule's own
   `CAUSE`, CHANGEABLE_RULES §6.4). A missing value stops the worker mid-job.
5. **Gate.** Run the full suite yourself, then push.
6. **Report.** In `report.md` and your final message: the ranked table, the steps with commit
   hashes, what was deferred and why, what you took on the workers' word only, the tiers used,
   and wall time. End with "committed and pushed on the current branch `<name>`".

## Hard stops (the only reasons to stop and ask)

- A model is missing or the opencode quota is reached: never swap models on your own.
- The full suite is red after one escalation and the only fix is to revert a commit: revert
  it, keep going, and flag it in the report (this is a stop for that item, not for the run).

Everything else, including "nothing worth implementing today", is your decision; write it in
the report.
